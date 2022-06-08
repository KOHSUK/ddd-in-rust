-- Add migration script here
CREATE TABLE IF NOT EXISTS public.club (
    club_id UUID NOT NULL,
    user_id UUID NOT NULL,
    PRIMARY KEY (club_id, user_id)
);

