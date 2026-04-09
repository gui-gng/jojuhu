-- Scheduled Posts
-- Allows users to schedule posts for future publication
CREATE TABLE IF NOT EXISTS scheduled_posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    media_urls TEXT[],
    scheduled_for TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) DEFAULT 'pending', -- pending, published, failed, cancelled
    published_at TIMESTAMPTZ,
    published_post_id UUID REFERENCES posts(id) ON DELETE SET NULL,
    error_message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_scheduled_posts_user ON scheduled_posts(user_id);
CREATE INDEX idx_scheduled_posts_status ON scheduled_posts(status);
CREATE INDEX idx_scheduled_posts_scheduled_for ON scheduled_posts(scheduled_for);

-- Draft Posts
-- Auto-save drafts for users
CREATE TABLE IF NOT EXISTS post_drafts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT,
    media_urls TEXT[],
    visibility VARCHAR(20) DEFAULT 'public',
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id) -- One draft per user
);

CREATE INDEX idx_post_drafts_user ON post_drafts(user_id);

-- GDPR Data Export Requests
CREATE TABLE IF NOT EXISTS data_export_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(20) DEFAULT 'pending', -- pending, processing, completed, failed
    file_path TEXT,
    file_size BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ
);

CREATE INDEX idx_data_export_requests_user ON data_export_requests(user_id);
CREATE INDEX idx_data_export_requests_status ON data_export_requests(status);

-- Account Deletion Requests
CREATE TABLE IF NOT EXISTS account_deletion_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reason TEXT,
    status VARCHAR(20) DEFAULT 'pending', -- pending, processing, completed, cancelled
    requested_at TIMESTAMPTZ DEFAULT NOW(),
    scheduled_deletion_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    UNIQUE(user_id)
);

CREATE INDEX idx_account_deletion_requests_user ON account_deletion_requests(user_id);
CREATE INDEX idx_account_deletion_requests_status ON account_deletion_requests(status);

-- Privacy Settings
CREATE TABLE IF NOT EXISTS privacy_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE UNIQUE,
    profile_visibility VARCHAR(20) DEFAULT 'public', -- public, followers_only, private
    show_email BOOLEAN DEFAULT FALSE,
    show_phone BOOLEAN DEFAULT FALSE,
    allow_mentions BOOLEAN DEFAULT TRUE,
    allow_tags BOOLEAN DEFAULT TRUE,
    show_online_status BOOLEAN DEFAULT TRUE,
    show_activity BOOLEAN DEFAULT TRUE,
    allow_search_engines BOOLEAN DEFAULT FALSE,
    data_processing_consent BOOLEAN DEFAULT FALSE,
    marketing_emails_consent BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_privacy_settings_user ON privacy_settings(user_id);

-- Push Notification Tokens
CREATE TABLE IF NOT EXISTS push_notification_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_token TEXT NOT NULL,
    device_type VARCHAR(20) NOT NULL, -- ios, android, web
    device_name TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    last_used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, device_token)
);

CREATE INDEX idx_push_notification_tokens_user ON push_notification_tokens(user_id);
CREATE INDEX idx_push_notification_tokens_device ON push_notification_tokens(device_token);