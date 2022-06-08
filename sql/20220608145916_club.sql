-- Add migration script here
CREATE TABLE IF NOT EXISTS public.club (
    id UUID NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

alter table public.club add unique(id);