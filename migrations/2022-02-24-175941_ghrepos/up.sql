
DROP TABLE IF EXISTS sites;

CREATE TABLE repos (
    id SERIAL PRIMARY KEY,
    repo_name TEXT NOT NULL,
    repo_url TEXT NOT NULL UNIQUE,
    languages_id INTEGER[] NOT NULL,
    contributors_id INTEGER[] NOT NULL
);

CREATE TABLE languages (
    id SERIAL PRIMARY KEY,
    language_name VARCHAR NOT NULL
);

CREATE TABLE contributors (
    id SERIAL PRIMARY KEY,
    contributor VARCHAR NOT NULL,
    contributor_url TEXT NOT NULL
);
