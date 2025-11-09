CREATE TABLE files (
                       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       user_id UUID REFERENCES users(id),
                       filename VARCHAR(255) NOT NULL,
                       original_name VARCHAR(255) NOT NULL,
                       size BIGINT NOT NULL,
                       uploaded_at TIMESTAMP DEFAULT NOW()
);