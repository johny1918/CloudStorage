use crate::auth::AuthUser;
use crate::database::AppState;
use axum::extract::{Extension, Request};
use axum::{Json, extract::State, http::StatusCode};
use axum::body::Body;
use multer::Multipart;
use bytes::Bytes;
use serde_json::json;
use uuid::Uuid;

pub async fn list_files(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Get user's files from database
    let files = sqlx::query!(
        "SELECT id, filename, original_name, size, uploaded_at FROM files WHERE user_id = $1 ORDER BY uploaded_at DESC",
        auth_user.user_id
    )
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let file_list: Vec<serde_json::Value> = files
        .into_iter()
        .map(|file| {
            json!({
                "id": file.id,
                "filename": file.filename,
                "original_name": file.original_name,
                "size": file.size,
                "uploaded_at": file.uploaded_at
            })
        })
        .collect();

    Ok(Json(json!({
        "status": "success",
        "files": file_list
    })))
}

pub async fn upload_file(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    request: Request<Body>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Create user directory
    let user_dir = format!("uploads/{}", auth_user.user_id);
    tokio::fs::create_dir_all(&user_dir)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create directory: {}", e)))?;

    // Get content type
    let content_type = request.headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .ok_or((StatusCode::BAD_REQUEST, "Missing content-type".to_string()))?;

    // Parse the multipart boundary
    let boundary = multer::parse_boundary(content_type)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid content-type: {}", e)))?;

    // Convert the request body to bytes
    let body_bytes = axum::body::to_bytes(request.into_body(), 10_000_000) // 10MB limit
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to read body: {}", e)))?;

    // Create multipart parser
    let mut multipart = Multipart::with_reader(body_bytes.as_ref(), boundary);

    let mut saved_file = None;

    // Process each field in the multipart form
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (StatusCode::BAD_REQUEST, format!("Failed to read multipart field: {}", e))
    })? {
        let field_name = field.name().unwrap_or("unknown").to_string();

        if field_name == "file" {
            // Get the original filename
            let original_filename = field.file_name()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "unknown".to_string());

            // Get the file content
            let file_data = field.bytes().await.map_err(|e| {
                (StatusCode::BAD_REQUEST, format!("Failed to read file data: {}", e))
            })?;

            // Generate unique filename
            let file_id = Uuid::new_v4();
            let stored_filename = format!("{}_{}", file_id, original_filename);
            let file_path = format!("{}/{}", user_dir, stored_filename);

            // Save file to disk
            tokio::fs::write(&file_path, &file_data)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to save file: {}", e)))?;

            // Save file metadata to database
            let file_record = sqlx::query!(
                "INSERT INTO files (user_id, filename, original_name, size) VALUES ($1, $2, $3, $4) RETURNING id, filename, original_name, size, uploaded_at",
                auth_user.user_id,
                stored_filename,
                original_filename,
                file_data.len() as i64
            )
                .fetch_one(&state.db)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            saved_file = Some(json!({
                "id": file_record.id,
                "filename": file_record.filename,
                "original_name": file_record.original_name,
                "size": file_record.size,
                "uploaded_at": file_record.uploaded_at
            }));

            break; 
        }
    }

    match saved_file {
        Some(file) => Ok(Json(json!({
            "status": "success",
            "message": "File uploaded successfully!",
            "file": file
        }))),
        None => Err((StatusCode::BAD_REQUEST, "No file found in request".to_string())),
    }
}
