# ☁️ Cloud Storage API

A secure, high-performance file storage API built with **Rust**, **Axum**, and **PostgreSQL**. This project demonstrates modern backend development with async/await, JWT authentication, and robust error handling.

## 🚀 Features

### 🔐 Authentication & Security
- **JWT-based authentication** with secure token generation
- **Password hashing** using bcrypt
- **Protected routes** with authentication middleware
- **User registration** and login system

### 📁 File Management
- **File upload** with multipart form data support
- **Secure file storage** with unique filenames (UUID + original name)
- **File metadata storage** in PostgreSQL
- **File download** with proper content headers
- **File listing** with detailed file information
- **File deletion** with automatic cleanup (filesystem + database)

### 🛡️ Security & Validation
- **File size limits** (5MB default, configurable)
- **File type validation** (blocks dangerous extensions)
- **User ownership verification** for all file operations
- **Input validation** and proper error handling

### 💻 Technical Features
- **Async/await** throughout the codebase
- **Custom error handling** with type-safe error enum
- **CORS support** for web frontend integration
- **Static file serving** for web interface
- **Database connection pooling**

## 📋 API Endpoints

### Authentication Endpoints

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| `POST` | `/api/register` | Register new user | ❌ |
| `POST` | `/api/login` | Login and get JWT token | ❌ |

### File Management Endpoints

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| `GET` | `/api/files` | List user's files | ✅ |
| `POST` | `/api/upload` | Upload new file | ✅ |
| `GET` | `/api/files/{id}` | Download specific file | ✅ |
| `DELETE` | `/api/files/{id}` | Delete specific file | ✅ |

## 🛠️ Installation & Setup

### Prerequisites
- **Rust** (latest stable version)
- **PostgreSQL** (version 12+)
- **Cargo** (comes with Rust)

### 1. Clone and Setup
```bash
git clone <your-repo-url>
cd CloudStorage
```

## Database Setup

```
-- Create database
CREATE DATABASE cloud_storage;

-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Create files table with CASCADE delete
CREATE TABLE files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    filename VARCHAR(255) NOT NULL,
    original_name VARCHAR(255) NOT NULL,
    size BIGINT NOT NULL,
    uploaded_at TIMESTAMP DEFAULT NOW()
);
```

## Environment Configuration

Create a `.env` file in the root directory and add the following:
```
DATABASE_URL=postgres://username:password@localhost/cloud_storage
JWT_SECRET=your_super_secret_jwt_key_here_make_it_long_and_secure
```

## Build and Run
```
# Install dependencies and build
cargo build

# Run the server
cargo run

# Server will start on http://localhost:3000
```

## Usage Examples
User Registration:
```
curl -X POST http://localhost:3000/api/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com",
    "password": "securepassword123"
  }'
```

User Login:
```
curl -X POST http://localhost:3000/api/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "password": "securepassword123"
  }'
  ```

File Upload:
```
curl -X POST http://localhost:3000/api/upload \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -F "file=@/path/to/your/file.pdf"
```

List Files:
```
curl -X GET http://localhost:3000/api/files \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

Download File:
```
curl -X GET http://localhost:3000/api/files/c15e6198-4dad-4182-8c8c-3bc35cec3d2e \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -o downloaded_file.pdf
```

Delete File:
```
curl -X DELETE http://localhost:3000/api/files/c15e6198-4dad-4182-8c8c-3bc35cec3d2e \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## Web Interface
```
Accessing the Web Interface

Start the server: cargo run

Open http://localhost:3000 in your browser

No additional setup required!
```

## File Type Restrictions
```
Blocked extensions: .exe, .bat, .cmd, .sh, .php

Prevents upload of potentially dangerous files

Easily extensible for custom restrictions
```

## 📄 License

This project is open source and available under the MIT License.