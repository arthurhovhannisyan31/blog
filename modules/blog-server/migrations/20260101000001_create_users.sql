CREATE TABLE IF NOT EXISTS users
(
    id            BIGSERIAL PRIMARY KEY,
    username      TEXT         NOT NULL,
    email         TEXT         NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_name_len CHECK (length(username) <= 255),
    CONSTRAINT chk_email_len CHECK (length(email) <= 255),
    CONSTRAINT chk_password_hash_len CHECK (length(password_hash) <= 255)
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users (username);