-- Add migration script here
ALTER TABLE club ADD COLUMN owner UUID; 

ALTER TABLE public.club ADD FOREIGN KEY (owner) REFERENCES public.user (id);

ALTER TABLE public.club_members ADD FOREIGN KEY (club_id) REFERENCES public.club (id);

ALTER TABLE public.club_members ADD FOREIGN KEY (user_id) REFERENCES public.user (id);

