use rusqlite::{Connection, Result};

#[derive(Debug)]
struct ToDo{
    id: i64,
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug)]
struct Status {
    is_completed: bool,
}

fn main() -> Result<()> {
    let conn = Connection::open("todo.db")?;

    conn.execute(
    "CREATE TABLE IF NOT EXISTS todo(\
    id INTEGER PRIMARY KEY,\
    title TEXT NOT NULL,\
    description TEXT NOT NULL,\
    is_completed INTEGER NOT NULL\
    )",
    (),
    )?;

    let todo = ToDo{
        id: 0,
        title: "Test todo".to_string(),
        description: "I should learn rust better!".to_string(),
        status: Status{is_completed: false},
    };

    conn.execute(
        "INSERT INTO todo (title, description, is_completed) VALUES (?1, ?2, ?3)",
        (&todo.title, &todo.description, &todo.status.is_completed),
    )?;

    let mut stmt = conn.prepare("SELECT id, title, description, is_completed FROM todo")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(ToDo{
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            status: Status{is_completed: row.get(3)?},
        })
    })?;

    for todo in todo_iter{
        let todo = todo?;
        println!("Found todo #{} {:?}", todo.id, todo);
    }
    Ok(())
}
