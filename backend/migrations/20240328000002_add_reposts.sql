-- Create reposts table
CREATE TABLE IF NOT EXISTS reposts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    original_post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    reposter_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    quote_text TEXT, -- Optional quote text when reposting with comment
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(original_post_id, reposter_id) -- Prevent duplicate reposts by same user
);

CREATE INDEX idx_reposts_original ON reposts(original_post_id);
CREATE INDEX idx_reposts_user ON reposts(reposter_id);
CREATE INDEX idx_reposts_created ON reposts(created_at DESC);

-- Update posts table to track repost count
ALTER TABLE posts ADD COLUMN IF NOT EXISTS reposts_count INTEGER NOT NULL DEFAULT 0;