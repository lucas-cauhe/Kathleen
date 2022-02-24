
CREATE TABLE sites (
    id SERIAL PRIMARY KEY,
    site_name TEXT NOT NULL,
    site_url TEXT NOT NULL UNIQUE
)