-- Add category to forums
ALTER TABLE forums ADD COLUMN IF NOT EXISTS category VARCHAR(100);

-- Add rules and guidelines to forums
ALTER TABLE forums ADD COLUMN IF NOT EXISTS rules TEXT[];

-- Add tags to topics
ALTER TABLE topics ADD COLUMN IF NOT EXISTS tags TEXT[];

-- Create index for category queries
CREATE INDEX IF NOT EXISTS idx_forums_category ON forums(category);

-- Create index for topic tags (for JSON array search)
CREATE INDEX IF NOT EXISTS idx_topics_tags ON topics USING GIN(tags);

-- Create forum bans table
CREATE TABLE IF NOT EXISTS forum_bans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    forum_id UUID NOT NULL REFERENCES forums(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    banned_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reason TEXT,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPT NOT NULL DEFAULT NOW(),
    UNIQUE(forum_id, user_id)
);

CREATE INDEX idx_forum_bans_forum ON forum_bans(forum_id);
CREATE INDEX idx_forum_bans_user ON forum_bans(user_id);

-- Create forum analytics table
CREATE TABLE IF NOT EXISTS forum_analytics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    forum_id UUID NOT NULL REFERENCES forums(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    views INTEGER NOT NULL DEFAULT 0,
    unique_visitors INTEGER NOT NULL DEFAULT 0,
    new_topics INTEGER NOT NULL DEFAULT 0,
    new_replies INTEGER NOT NULL DEFAULT 0,
    new_members INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPT NOT NULL DEFAULT NOW(),
    UNIQUE(forum_id, date)
);

CREATE INDEX idx_forum_analytics_forum ON forum_analytics(forum_id);
CREATE INDEX idx_forum_analytics_date ON forum_analytics(date);

-- Create topic analytics table
CREATE TABLE IF NOT EXISTS topic_views (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    topic_id UUID NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    viewed_at TIMESTAMPT NOT NULL DEFAULT NOW(),
    ip_address INET
);

CREATE INDEX idx_topic_views_topic ON topic_views(topic_id);
CREATE INDEX idx_topic_views_user ON topic_views(user_id);
CREATE INDEX idx_topic_views_viewed_at ON topic_views(viewed_at);