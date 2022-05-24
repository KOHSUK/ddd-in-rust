-- Add migration script here
CREATE TABLE IF NOT EXISTS public.user (
    id UUID NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);