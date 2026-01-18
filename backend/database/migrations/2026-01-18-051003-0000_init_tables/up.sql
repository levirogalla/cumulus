CREATE TABLE media (
    key VARCHAR PRIMARY KEY,
    notes VARCHAR,
    date_uploaded DATE
);

CREATE TABLE albums (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    date_created DATE
);

CREATE TABLE albums_media (
    id SERIAL PRIMARY KEY,
    media VARCHAR NOT NULL REFERENCES media(key) ON DELETE CASCADE,
    album INTEGER NOT NULL REFERENCES albums(id) ON DELETE CASCADE,
    date_added DATE
);