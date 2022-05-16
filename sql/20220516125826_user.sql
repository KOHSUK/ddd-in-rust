-- Add migration script here
CREATE TABLE IF NOT EXISTS public.user (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL
);