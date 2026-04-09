-- Content Moderation System

-- Create report types enum
CREATE TYPE report_type AS ENUM ('post', 'comment', 'forum_topic', 'forum_reply', 'user_profile');

-- Create report status enum
CREATE TYPE report_status AS ENUM ('pending', 'reviewing', 'resolved', 'dismissed');

-- Create moderation action types enum
CREATE TYPE moderation_action_type AS ENUM (
    'content_removed',
    'content_restored',
    'user_warned',
    'user_suspended',
    'user_banned',
    'content_hidden',
    'report_dismissed'
);

-- Create reports table
CREATE TABLE IF NOT EXISTS reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reporter_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reported_user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    report_type report_type NOT NULL,
    content_id UUID NOT NULL,
    reason TEXT NOT NULL,
    description TEXT,
    status report_status NOT NULL DEFAULT 'pending',
    assigned_to UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create moderation actions table
CREATE TABLE IF NOT EXISTS moderation_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_id UUID REFERENCES reports(id) ON DELETE SET NULL,
    moderator_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    action_type moderation_action_type NOT NULL,
    target_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    target_content_id UUID,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create user suspensions table
CREATE TABLE IF NOT EXISTS user_suspensions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    suspended_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reason TEXT NOT NULL,
    ends_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_reports_reporter ON reports(reporter_id);
CREATE INDEX IF NOT EXISTS idx_reports_reported_user ON reports(reported_user_id);
CREATE INDEX IF NOT EXISTS idx_reports_status ON reports(status);
CREATE INDEX IF NOT EXISTS idx_reports_created_at ON reports(created_at);
CREATE INDEX IF NOT EXISTS idx_reports_assigned_to ON reports(assigned_to);

CREATE INDEX IF NOT EXISTS idx_moderation_actions_moderator ON moderation_actions(moderator_id);
CREATE INDEX IF NOT EXISTS idx_moderation_actions_report ON moderation_actions(report_id);
CREATE INDEX IF NOT EXISTS idx_moderation_actions_created_at ON moderation_actions(created_at);

CREATE INDEX IF NOT EXISTS idx_user_suspensions_user ON user_suspensions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_suspensions_ends_at ON user_suspensions(ends_at);