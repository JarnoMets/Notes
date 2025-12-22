//! Core database types, error handling, and connection management

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    QueryError(#[from] sqlx::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Not found")]
    NotFound,
    #[error("Invalid data: {0}")]
    #[allow(dead_code)]
    InvalidData(String),
    #[error("Email already exists")]
    EmailExists,
    #[error("Invalid credentials")]
    #[allow(dead_code)]
    InvalidCredentials,
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

pub type DbResult<T> = Result<T, DbError>;

#[derive(Clone)]
pub struct Database {
    pub(crate) pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> DbResult<Self> {
        // Parse the database URL to extract the database name
        let mut url = Url::parse(database_url)?;
        let db_name = url
            .path_segments()
            .and_then(|mut segments| segments.next())
            .unwrap_or("notesdb")
            .to_string();

        // Create a URL pointing to the default 'postgres' database
        url.set_path("/postgres");
        let admin_url = url.to_string();

        // Connect to the default postgres database first
        let admin_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&admin_url)
            .await?;

        // Check if our target database exists
        let db_exists: bool = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)"
        )
        .bind(&db_name)
        .fetch_one(&admin_pool)
        .await?
        .get(0);

        if !db_exists {
            log::info!("Database '{}' does not exist, creating it...", db_name);
            // Note: CREATE DATABASE cannot be run in a transaction, and we can't use
            // parameterized queries for database names, so we need to be careful here
            let create_query = format!("CREATE DATABASE \"{}\"", db_name.replace('"', "\"\""));
            sqlx::query(&create_query)
                .execute(&admin_pool)
                .await?;
            log::info!("Database '{}' created successfully", db_name);
        }

        // Close the admin connection
        admin_pool.close().await;

        // Now connect to the actual database
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Self::run_migrations(&pool).await?;

        Ok(Database { pool })
    }

    async fn run_migrations(pool: &PgPool) -> DbResult<()> {
        // Users table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                password_hash TEXT,
                name TEXT NOT NULL,
                google_id TEXT UNIQUE,
                avatar_url TEXT,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Notes table with user_id
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                title TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                content TEXT NOT NULL DEFAULT '',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Add user_id column if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE notes ADD COLUMN IF NOT EXISTS user_id TEXT REFERENCES users(id) ON DELETE CASCADE"
        )
        .execute(pool)
        .await;

        // Add description column if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE notes ADD COLUMN IF NOT EXISTS description TEXT NOT NULL DEFAULT ''"
        )
        .execute(pool)
        .await;

        // Add folder_id column if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE notes ADD COLUMN IF NOT EXISTS folder_id TEXT"
        )
        .execute(pool)
        .await;

        // Add position column if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE notes ADD COLUMN IF NOT EXISTS position INTEGER NOT NULL DEFAULT 0"
        )
        .execute(pool)
        .await;

        // Add is_important column to notes if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE notes ADD COLUMN IF NOT EXISTS is_important BOOLEAN NOT NULL DEFAULT FALSE"
        )
        .execute(pool)
        .await;

        // Add is_urgent column to notes if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE notes ADD COLUMN IF NOT EXISTS is_urgent BOOLEAN NOT NULL DEFAULT FALSE"
        )
        .execute(pool)
        .await;

        // Note folders table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS note_folders (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                parent_id TEXT REFERENCES note_folders(id) ON DELETE CASCADE,
                name TEXT NOT NULL,
                position INTEGER NOT NULL DEFAULT 0,
                is_important BOOLEAN NOT NULL DEFAULT FALSE,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Add is_important column to note_folders if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE note_folders ADD COLUMN IF NOT EXISTS is_important BOOLEAN NOT NULL DEFAULT FALSE"
        )
        .execute(pool)
        .await;

        // Add is_urgent column to note_folders if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE note_folders ADD COLUMN IF NOT EXISTS is_urgent BOOLEAN NOT NULL DEFAULT FALSE"
        )
        .execute(pool)
        .await;

        // Note attachments table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS note_attachments (
                id TEXT PRIMARY KEY,
                note_id TEXT NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
                filename TEXT NOT NULL,
                original_filename TEXT NOT NULL,
                mime_type TEXT NOT NULL,
                size BIGINT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Note revisions table (diff-based revisions) and current revision index on notes
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS note_revisions (
                id TEXT PRIMARY KEY,
                note_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                idx INTEGER NOT NULL,
                forward_patch TEXT NOT NULL,
                reverse_patch TEXT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )"
        )
        .execute(pool)
        .await?;

        // Index for fast lookup by note and index
        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_note_revisions_note_idx ON note_revisions (note_id, idx)")
            .execute(pool)
            .await;

        // Add current_revision_index to notes (tracks the applied revision index)
        let _ = sqlx::query("ALTER TABLE notes ADD COLUMN IF NOT EXISTS current_revision_index INTEGER DEFAULT -1")
            .execute(pool)
            .await;

        // Boards table with user_id
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS boards (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                name TEXT NOT NULL,
                description TEXT,
                color TEXT,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Add user_id column if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE boards ADD COLUMN IF NOT EXISTS user_id TEXT REFERENCES users(id) ON DELETE CASCADE"
        )
        .execute(pool)
        .await;

        // Add folder_id column to boards if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE boards ADD COLUMN IF NOT EXISTS folder_id TEXT"
        )
        .execute(pool)
        .await;

        // Add position column to boards if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE boards ADD COLUMN IF NOT EXISTS position INTEGER NOT NULL DEFAULT 0"
        )
        .execute(pool)
        .await;

        // Add is_important column to boards if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE boards ADD COLUMN IF NOT EXISTS is_important BOOLEAN NOT NULL DEFAULT FALSE"
        )
        .execute(pool)
        .await;

        // Add is_urgent column to boards if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE boards ADD COLUMN IF NOT EXISTS is_urgent BOOLEAN NOT NULL DEFAULT FALSE"
        )
        .execute(pool)
        .await;

        // Board folders table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS board_folders (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                parent_id TEXT REFERENCES board_folders(id) ON DELETE CASCADE,
                name TEXT NOT NULL,
                position INTEGER NOT NULL DEFAULT 0,
                is_important BOOLEAN NOT NULL DEFAULT FALSE,
                is_urgent BOOLEAN NOT NULL DEFAULT FALSE,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Add is_important column to board_folders if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE board_folders ADD COLUMN IF NOT EXISTS is_important BOOLEAN NOT NULL DEFAULT FALSE"
        )
        .execute(pool)
        .await;

        // Add is_urgent column to board_folders if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE board_folders ADD COLUMN IF NOT EXISTS is_urgent BOOLEAN NOT NULL DEFAULT FALSE"
        )
        .execute(pool)
        .await;

        // Add foreign key to boards for folder_id after board_folders table exists
        // Note: We can't add FK constraint with ALTER TABLE IF NOT EXISTS easily, so we just ensure the column exists

        // Lists table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS lists (
                id TEXT PRIMARY KEY,
                board_id TEXT NOT NULL REFERENCES boards(id) ON DELETE CASCADE,
                name TEXT NOT NULL,
                position INTEGER NOT NULL DEFAULT 0,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Cards table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS cards (
                id TEXT PRIMARY KEY,
                list_id TEXT NOT NULL REFERENCES lists(id) ON DELETE CASCADE,
                title TEXT NOT NULL,
                description TEXT,
                position INTEGER NOT NULL DEFAULT 0,
                due_date TIMESTAMPTZ,
                labels JSONB NOT NULL DEFAULT '[]',
                archived BOOLEAN NOT NULL DEFAULT FALSE,
                status TEXT NOT NULL DEFAULT 'open',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Add archived column to cards if not exists
        let _ = sqlx::query(
            "ALTER TABLE cards ADD COLUMN IF NOT EXISTS archived BOOLEAN NOT NULL DEFAULT FALSE"
        )
        .execute(pool)
        .await;

        // Add status column to cards if not exists (migration)
        let _ = sqlx::query(
            "ALTER TABLE cards ADD COLUMN IF NOT EXISTS status TEXT NOT NULL DEFAULT 'open'"
        )
        .execute(pool)
        .await;

        // Add archived column to lists if not exists
        let _ = sqlx::query(
            "ALTER TABLE lists ADD COLUMN IF NOT EXISTS archived BOOLEAN NOT NULL DEFAULT FALSE"
        )
        .execute(pool)
        .await;

        // Board labels table (predefined labels for a board)
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS board_labels (
                id TEXT PRIMARY KEY,
                board_id TEXT NOT NULL REFERENCES boards(id) ON DELETE CASCADE,
                name TEXT NOT NULL,
                color TEXT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Automation rules table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS automation_rules (
                id TEXT PRIMARY KEY,
                board_id TEXT NOT NULL REFERENCES boards(id) ON DELETE CASCADE,
                name TEXT NOT NULL,
                enabled BOOLEAN NOT NULL DEFAULT TRUE,
                trigger_type TEXT NOT NULL,
                trigger_config JSONB NOT NULL DEFAULT '{}',
                action_type TEXT NOT NULL,
                action_config JSONB NOT NULL DEFAULT '{}',
                last_run_at TIMESTAMPTZ,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // User settings table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS user_settings (
                user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
                theme TEXT NOT NULL DEFAULT 'dark',
                week_starts_on_monday BOOLEAN NOT NULL DEFAULT TRUE,
                ics_calendars JSONB NOT NULL DEFAULT '[]',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Reminders table for calendar reminders and note links
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS reminders (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                note_id TEXT NOT NULL DEFAULT '',
                title TEXT NOT NULL,
                date TEXT NOT NULL,
                time TEXT NOT NULL DEFAULT '09:00',
                remind_before TEXT NOT NULL DEFAULT '0',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Indexes
        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_google_id ON users(google_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_notes_user ON notes(user_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_notes_updated ON notes(updated_at DESC)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_note_attachments_note ON note_attachments(note_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_note_folders_user ON note_folders(user_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_note_folders_parent ON note_folders(parent_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_notes_folder ON notes(folder_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_boards_user ON boards(user_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_boards_folder ON boards(folder_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_board_folders_user ON board_folders(user_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_board_folders_parent ON board_folders(parent_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_lists_board ON lists(board_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_lists_position ON lists(board_id, position)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_cards_list ON cards(list_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_cards_position ON cards(list_id, position)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_cards_archived ON cards(list_id, archived)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_cards_status ON cards(list_id, status)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_board_labels ON board_labels(board_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_automation_rules ON automation_rules(board_id)")
            .execute(pool)
            .await;

        // Add last_run_at column if it doesn't exist (migration)
        let _ = sqlx::query("ALTER TABLE automation_rules ADD COLUMN IF NOT EXISTS last_run_at TIMESTAMPTZ")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_reminders_user ON reminders(user_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_reminders_date ON reminders(user_id, date)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_reminders_note ON reminders(note_id)")
            .execute(pool)
            .await;

        Ok(())
    }
}
