CREATE TABLE IF NOT EXISTS Posts
(
    id         BIGSERIAL PRIMARY KEY,
    title      VARCHAR     NOT NULL,
    content    TEXT        NOT NULL,
    author_id  BIGINT      NOT NULL REFERENCES blog_db.public.users (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ          DEFAULT NULL
);

CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts (created_at);
CREATE INDEX IF NOT EXISTS idx_posts_author_id ON posts (author_id);
CREATE INDEX IF NOT EXISTS idx_posts_active ON posts (id) WHERE deleted_at IS NULL;