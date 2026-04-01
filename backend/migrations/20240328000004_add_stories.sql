-- Create stories table for ephemeral content (24-hour disappearing stories)
CREATE TABLE IF NOT EXISTS stories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    media_url TEXT NOT NULL,
    media_type VARCHAR(20) NOT NULL DEFAULT 'image', -- 'image', 'video', 'text'
    caption TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '24 hours'),
    view_count INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_stories_user ON stories(user_id);
CREATE INDEX idx_stories_expires ON stories(expires_at);

-- Create story viewers table
CREATE TABLE IF NOT EXISTS story_viewers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    story_id UUID NOT NULL REFERENCES stories(id) ON DELETE CASCADE,
    viewer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    viewed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reaction VARCHAR(20), -- 'like', 'heart', 'laugh', 'wow', 'sad', 'angry'
    UNIQUE(story_id, viewer_id)
);

CREATE INDEX idx_story_viewers_story ON story_viewers(story_id);
CREATE INDEX idx_story_viewers_viewer ON story_viewers(viewer_id);

-- Add stories_count to users table
ALTER TABLE users ADD COLUMN IF NOT EXISTS stories_count INTEGER NOT NULL DEFAULT 0;