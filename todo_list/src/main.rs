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

fn conn() -> Result<Connection> {
    let conn = Connection::open("todos.db")?;
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
}

fn todo_model(title: String, description: String, is_completed: bool) -> Result<String> {
    let todo = ToDo{
        id: len(id)+1,
        title: title.to_string(),
        description: description.to_string(),
        status: Status{is_completed: false},
    };
}

fn get_todos(){}

fn get_todo(){}

fn insert_todo(){}

fn update_todo(){}

fn delete_todo(){}

fn main() -> Result<()> {

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
