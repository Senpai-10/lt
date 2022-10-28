use rusqlite::Connection;

pub mod tasks;

/// CREATE TABLE/S
pub fn setup(conn: &Connection) -> () {
    // find a way to ALTER table and add missing columns
    conn.execute(
        r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id                  VARCHAR     NOT NULL PRIMARY KEY        ,
                category            VARCHAR     NOT NULL                    ,
                text                TEXT        NOT NULL                    ,
                status              TEXT        NOT NULL                    ,
                priority            INT         NOT NULL DEFAULT(1)         ,
                creation_date       INT         NOT NULL                    ,
                completion_date     INT         NULL     DEFAULT(NULL)      ,
                modification_date   INT         NULL     DEFAULT(NULL)
            )
        "#,
        (),
    )
    .unwrap();
}
