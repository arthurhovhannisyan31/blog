CREATE TABLE IF NOT EXISTS Posts
(
    id         BIGSERIAL PRIMARY KEY,
    title      VARCHAR     NOT NULL,
    content    TEXT,
    author_id  BIGINT REFERENCES blog_db.public.users (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_posts_created_at ON Posts (created_at);
CREATE INDEX IF NOT EXISTS idx_posts_author_id ON Posts (author_id);