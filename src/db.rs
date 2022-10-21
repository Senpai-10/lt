use rusqlite::Connection;

pub mod tasks;

/// CREATE TABLE/S
pub fn setup(conn: &Connection) -> Result<usize, rusqlite::Error> {
    // find a way to ALTER table and add missing columns
    conn.execute(
        r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id          VARCHAR     NOT NULL PRIMARY KEY        ,
                category    VARCHAR     NOT NULL                    ,
                text        TEXT        NOT NULL                    ,
                is_done     BOOLEAN     NOT NULL DEFAULT('false')   ,
                priority    INT         NOT NULL DEFAULT(1)         ,
                done_date   INT         NULL     DEFAULT(NULL)
            )
        "#,
        (),
    )
}
