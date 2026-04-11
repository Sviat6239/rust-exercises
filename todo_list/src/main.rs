use rusqlite::{Connection, Result};

#[derive(Debug)]
struct ToDo{
    id: i64,
    title: String,
    description: String,
    is_completed: bool,
}


fn conn() -> Result<Connection> {
    let conn = Connection::open("todos.db")?;
    Ok(conn)
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo(\
    id INTEGER PRIMARY KEY,\
    title TEXT NOT NULL,\
    description TEXT NOT NULL,\
    is_completed INTEGER NOT NULL\
    )",
        (),
    )?;
    Ok(())
}

fn get_todos(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, title, description, is_completed FROM todo")?;

    let todo_iter = stmt.query_map([], |row| {
        Ok(ToDo{
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            is_completed: row.get(3)?,
        })
    })?;

    for todo in todo_iter{
        let todo = todo?;
        println!("Found todo #{} {:?}", todo.id, todo);
    }
    Ok(())
}

fn get_todo(conn: &Connection, id: i64) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, title, description, is_completed FROM todo WHERE id = ?1")?;
    let todo_iter = stmt.query_map([id], |row| {
        Ok(ToDo{
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            is_completed: row.get(3)?,
        })
    })?;

    for todo in todo_iter{
        let todo = todo?;
        println!("{:?}", todo);
    }

    Ok(())
}

fn insert_todo(conn: &Connection, title: String, description: String, is_completed: bool) -> Result<()> {
    conn.execute(
        "INSERT INTO todo (title, description, is_completed) VALUES (?1, ?2, ?3)",
        (&title, &description, is_completed),
    )?;
    Ok(())
}

fn update_todo(){}

fn delete_todo(){}

fn main() -> Result<()>{
    let conn = conn()?;
    create_table(&conn)?;

    insert_todo(
        &conn,
        "Learn Rust".to_string(),
        "I should know Rust better".to_string(),
        false,
    )?;

    get_todos(&conn)?;

    get_todo(&conn, 1)?;

    Ok(())
}
