use crate::error::Result;
use crate::types::{Tag, Wallpaper, WallpaperType};
use chrono::DateTime;
use rusqlite::{params, Connection};
use std::path::Path;
use uuid::Uuid;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.initialize()?;
        Ok(db)
    }

    fn initialize(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS wallpapers (
                id TEXT PRIMARY KEY,
                path TEXT NOT NULL UNIQUE,
                filename TEXT NOT NULL,
                type TEXT NOT NULL,
                width INTEGER NOT NULL,
                height INTEGER NOT NULL,
                size INTEGER NOT NULL,
                hash TEXT NOT NULL UNIQUE,
                source TEXT,
                source_url TEXT,
                thumbnail_path TEXT,
                created_at TEXT NOT NULL,
                modified_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                category TEXT,
                count INTEGER DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS wallpaper_tags (
                wallpaper_id TEXT NOT NULL,
                tag_id INTEGER NOT NULL,
                PRIMARY KEY (wallpaper_id, tag_id),
                FOREIGN KEY (wallpaper_id) REFERENCES wallpapers(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS monitors (
                name TEXT PRIMARY KEY,
                width INTEGER NOT NULL,
                height INTEGER NOT NULL,
                x INTEGER NOT NULL,
                y INTEGER NOT NULL,
                is_primary INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS active_wallpapers (
                monitor TEXT,
                wallpaper_id TEXT NOT NULL,
                mode TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                PRIMARY KEY (monitor),
                FOREIGN KEY (wallpaper_id) REFERENCES wallpapers(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_wallpapers_type ON wallpapers(type);
            CREATE INDEX IF NOT EXISTS idx_wallpapers_source ON wallpapers(source);
            CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name);
            CREATE INDEX IF NOT EXISTS idx_wallpaper_tags_wallpaper ON wallpaper_tags(wallpaper_id);
            CREATE INDEX IF NOT EXISTS idx_wallpaper_tags_tag ON wallpaper_tags(tag_id);
            "#,
        )?;
        Ok(())
    }

    pub fn add_wallpaper(&self, wallpaper: &Wallpaper) -> Result<()> {
        let type_str = match wallpaper.wallpaper_type {
            WallpaperType::Image => "image",
            WallpaperType::Video => "video",
            WallpaperType::Spine => "spine",
            WallpaperType::WallpaperEngine => "wallpaper_engine",
        };

        self.conn.execute(
            r#"
            INSERT INTO wallpapers
            (id, path, filename, type, width, height, size, hash, source, source_url,
             thumbnail_path, created_at, modified_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
            "#,
            params![
                wallpaper.id.to_string(),
                &wallpaper.path,
                &wallpaper.filename,
                type_str,
                wallpaper.width,
                wallpaper.height,
                wallpaper.size as i64,
                &wallpaper.hash,
                &wallpaper.source,
                &wallpaper.source_url,
                &wallpaper.thumbnail_path,
                wallpaper.created_at.to_rfc3339(),
                wallpaper.modified_at.to_rfc3339(),
            ],
        )?;

        // Add tags
        for tag_name in &wallpaper.tags {
            let tag_id = self.get_or_create_tag(tag_name, None)?;
            self.conn.execute(
                "INSERT OR IGNORE INTO wallpaper_tags (wallpaper_id, tag_id) VALUES (?1, ?2)",
                params![wallpaper.id.to_string(), tag_id],
            )?;
        }

        Ok(())
    }

    pub fn get_wallpaper(&self, id: &Uuid) -> Result<Wallpaper> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT id, path, filename, type, width, height, size, hash,
                   source, source_url, thumbnail_path, created_at, modified_at
            FROM wallpapers WHERE id = ?1
            "#,
        )?;

        let wallpaper = stmt.query_row(params![id.to_string()], |row| {
            let type_str: String = row.get(3)?;
            let wallpaper_type = match type_str.as_str() {
                "image" => WallpaperType::Image,
                "video" => WallpaperType::Video,
                "spine" => WallpaperType::Spine,
                "wallpaper_engine" => WallpaperType::WallpaperEngine,
                _ => WallpaperType::Image,
            };

            let id_str: String = row.get(0)?;
            let created_str: String = row.get(11)?;
            let modified_str: String = row.get(12)?;

            Ok(Wallpaper {
                id: Uuid::parse_str(&id_str).unwrap(),
                path: row.get(1)?,
                filename: row.get(2)?,
                wallpaper_type,
                width: row.get(4)?,
                height: row.get(5)?,
                size: row.get::<_, i64>(6)? as u64,
                hash: row.get(7)?,
                source: row.get(8)?,
                source_url: row.get(9)?,
                thumbnail_path: row.get(10)?,
                created_at: DateTime::parse_from_rfc3339(&created_str).unwrap().into(),
                modified_at: DateTime::parse_from_rfc3339(&modified_str).unwrap().into(),
                tags: Vec::new(),
            })
        })?;

        Ok(wallpaper)
    }

    pub fn list_wallpapers(&self, filter_type: Option<WallpaperType>) -> Result<Vec<Wallpaper>> {
        let query = if let Some(wtype) = filter_type {
            let type_str = match wtype {
                WallpaperType::Image => "image",
                WallpaperType::Video => "video",
                WallpaperType::Spine => "spine",
                WallpaperType::WallpaperEngine => "wallpaper_engine",
            };
            format!(
                "SELECT id, path, filename, type, width, height, size, hash,
                        source, source_url, thumbnail_path, created_at, modified_at
                 FROM wallpapers WHERE type = '{}' ORDER BY created_at DESC",
                type_str
            )
        } else {
            "SELECT id, path, filename, type, width, height, size, hash,
                    source, source_url, thumbnail_path, created_at, modified_at
             FROM wallpapers ORDER BY created_at DESC"
                .to_string()
        };

        let mut stmt = self.conn.prepare(&query)?;
        let wallpapers = stmt
            .query_map([], |row| {
                let type_str: String = row.get(3)?;
                let wallpaper_type = match type_str.as_str() {
                    "image" => WallpaperType::Image,
                    "video" => WallpaperType::Video,
                    "spine" => WallpaperType::Spine,
                    "wallpaper_engine" => WallpaperType::WallpaperEngine,
                    _ => WallpaperType::Image,
                };

                let id_str: String = row.get(0)?;
                let created_str: String = row.get(11)?;
                let modified_str: String = row.get(12)?;

                Ok(Wallpaper {
                    id: Uuid::parse_str(&id_str).unwrap(),
                    path: row.get(1)?,
                    filename: row.get(2)?,
                    wallpaper_type,
                    width: row.get(4)?,
                    height: row.get(5)?,
                    size: row.get::<_, i64>(6)? as u64,
                    hash: row.get(7)?,
                    source: row.get(8)?,
                    source_url: row.get(9)?,
                    thumbnail_path: row.get(10)?,
                    created_at: DateTime::parse_from_rfc3339(&created_str).unwrap().into(),
                    modified_at: DateTime::parse_from_rfc3339(&modified_str).unwrap().into(),
                    tags: Vec::new(),
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(wallpapers)
    }

    pub fn delete_wallpaper(&self, id: &Uuid) -> Result<()> {
        self.conn.execute(
            "DELETE FROM wallpapers WHERE id = ?1",
            params![id.to_string()],
        )?;
        Ok(())
    }

    pub fn get_or_create_tag(&self, name: &str, category: Option<&str>) -> Result<i64> {
        // Try to get existing tag
        if let Ok(id) = self.conn.query_row(
            "SELECT id FROM tags WHERE name = ?1",
            params![name],
            |row| row.get(0),
        ) {
            return Ok(id);
        }

        // Create new tag
        self.conn.execute(
            "INSERT INTO tags (name, category, count) VALUES (?1, ?2, 0)",
            params![name, category],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn search_tags(&self, prefix: &str, limit: usize) -> Result<Vec<Tag>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, category, count FROM tags WHERE name LIKE ?1 ORDER BY count DESC LIMIT ?2",
        )?;

        let tags = stmt
            .query_map(params![format!("{}%", prefix), limit as i64], |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    category: row.get(2)?,
                    count: row.get(3)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(tags)
    }

    pub fn get_wallpaper_tags(&self, wallpaper_id: &Uuid) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT t.name FROM tags t
            JOIN wallpaper_tags wt ON t.id = wt.tag_id
            WHERE wt.wallpaper_id = ?1
            "#,
        )?;

        let tags = stmt
            .query_map(params![wallpaper_id.to_string()], |row| row.get(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(tags)
    }

    pub fn search_by_tags(&self, tags: &[String]) -> Result<Vec<Wallpaper>> {
        if tags.is_empty() {
            return self.list_wallpapers(None);
        }

        let placeholders = tags.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!(
            r#"
            SELECT DISTINCT w.id, w.path, w.filename, w.type, w.width, w.height,
                   w.size, w.hash, w.source, w.source_url, w.thumbnail_path,
                   w.created_at, w.modified_at
            FROM wallpapers w
            JOIN wallpaper_tags wt ON w.id = wt.wallpaper_id
            JOIN tags t ON wt.tag_id = t.id
            WHERE t.name IN ({})
            GROUP BY w.id
            HAVING COUNT(DISTINCT t.name) = ?
            ORDER BY w.created_at DESC
            "#,
            placeholders
        );

        let mut stmt = self.conn.prepare(&query)?;
        let tags_len = tags.len();
        let params_vec: Vec<&dyn rusqlite::ToSql> = tags
            .iter()
            .map(|t| t as &dyn rusqlite::ToSql)
            .chain(std::iter::once(&tags_len as &dyn rusqlite::ToSql))
            .collect();

        let wallpapers = stmt
            .query_map(params_vec.as_slice(), |row| {
                let type_str: String = row.get(3)?;
                let wallpaper_type = match type_str.as_str() {
                    "image" => WallpaperType::Image,
                    "video" => WallpaperType::Video,
                    "spine" => WallpaperType::Spine,
                    "wallpaper_engine" => WallpaperType::WallpaperEngine,
                    _ => WallpaperType::Image,
                };

                let id_str: String = row.get(0)?;
                let created_str: String = row.get(11)?;
                let modified_str: String = row.get(12)?;

                Ok(Wallpaper {
                    id: Uuid::parse_str(&id_str).unwrap(),
                    path: row.get(1)?,
                    filename: row.get(2)?,
                    wallpaper_type,
                    width: row.get(4)?,
                    height: row.get(5)?,
                    size: row.get::<_, i64>(6)? as u64,
                    hash: row.get(7)?,
                    source: row.get(8)?,
                    source_url: row.get(9)?,
                    thumbnail_path: row.get(10)?,
                    created_at: DateTime::parse_from_rfc3339(&created_str).unwrap().into(),
                    modified_at: DateTime::parse_from_rfc3339(&modified_str).unwrap().into(),
                    tags: Vec::new(),
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(wallpapers)
    }
}
