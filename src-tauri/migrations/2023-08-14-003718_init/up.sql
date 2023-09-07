-- Your SQL goes here

CREATE TABLE categories (
    name                    VARCHAR(255) PRIMARY KEY NOT NULL
);

CREATE TABLE tasks (
    id                      VARCHAR PRIMARY KEY NOT NULL,
    category_name           VARCHAR(255) NOT NULL REFERENCES categories(name) ON DELETE CASCADE,
    title                   TEXT NOT NULL,
    desc                    TEXT,
    status                  INT NOT NULL,
    priority                INT NOT NULL DEFAULT(0),

    done_at                 INT,
    created_at              INT NOT NULL,
    updated_at              INT NOT NULL
);
