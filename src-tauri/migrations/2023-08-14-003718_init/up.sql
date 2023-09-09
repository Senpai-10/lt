-- Your SQL goes here

CREATE TABLE categories (
    name                        TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE tasks (
    id                          TEXT PRIMARY KEY NOT NULL,
    category_name               TEXT NOT NULL,
    title                       TEXT NOT NULL,
    desc                        TEXT,
    status                      INT NOT NULL,
    priority                    INT NOT NULL DEFAULT(0),
    is_child_task               INT NOT NULL,

    done_at                     INT,
    created_at                  INT NOT NULL,
    updated_at                  INT NOT NULL,

    FOREIGN KEY(category_name)  REFERENCES categories(name) ON DELETE CASCADE
);

CREATE TABLE subtasks (
    id                          TEXT PRIMARY KEY NOT NULL,
    parent_id                   TEXT NOT NULL,

    FOREIGN KEY(id)             REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY(parent_id)      REFERENCES tasks(id) ON DELETE CASCADE
);
