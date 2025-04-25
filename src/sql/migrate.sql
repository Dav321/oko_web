CREATE TABLE IF NOT EXISTS names (
    id INTEGER PRIMARY KEY,
    section_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    UNIQUE (section_id, name) ON CONFLICT IGNORE
);

CREATE TABLE IF NOT EXISTS sections (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    UNIQUE (name) ON CONFLICT IGNORE
);

CREATE TABLE IF NOT EXISTS vmix (
    id INTEGER PRIMARY KEY CHECK ( id = 0 ),
    url TEXT NOT NULL,
    overlay_index INTEGER NOT NULL CHECK ( overlay_index > 0 AND overlay_index < 5 ),
    object_uuid TEXT NOT NULL,
    name_field TEXT NOT NULL,
    title_field TEXT NOT NULL,
    UNIQUE (id) ON CONFLICT IGNORE
);

INSERT INTO vmix (id, url, overlay_index, object_uuid, name_field, title_field)
VALUES (0, 'localhost:8088', 1, 'uuid', 'name_field', 'title_field');

CREATE TABLE IF NOT EXISTS announcements (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    text TEXT NOT NULL,
    UNIQUE (name) ON CONFLICT IGNORE
);

CREATE TABLE IF NOT EXISTS propresenter (
    id INTEGER PRIMARY KEY CHECK ( id = 0 ),
    url TEXT NOT NULL,
    message_name TEXT NOT NULL,
    theme_name TEXT NOT NULL,
    theme_index INTEGER NOT NULL,
    theme_uuid TEXT NOT NULL,
    UNIQUE (id) ON CONFLICT IGNORE
);

INSERT INTO propresenter (id, url, message_name, theme_name, theme_index, theme_uuid)
VALUES (0, 'localhost:1025', 'API Message', 'theme_name', 0, 'theme_uuid')