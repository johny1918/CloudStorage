use sqlx::error::DatabaseError;
#[derive(Debug)]
pub enum AppError {
    // Database errors
    Database(sqlx::Error),

    // Authentication errors
    Unauthorized(String),
    InvalidCredentials,
    TokenCreation,
    TokenValidation,

    // User input errors
    BadRequest(String),
    NotFound(String),

    // File operations
    FileSystem(String),
    FileTooLarge,
    InvalidFileType,

    // Internal server errors
    Internal(String),
}