use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    title: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    userId: i32,
    id: i32,
    title: String,
    completed: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let article = Article {
        title: String::from("How to Work With JSON Data in Rust"),
        author: String::from("Matt T."),
        paragraph: vec![
            Paragraph {
                text: String::from("Rust is a systems programming language."),
            },
            Paragraph {
                text: String::from("It is fast and memory-efficient."),
            },
            Paragraph {
                text: String::from("Do some JSON stuff too."),
            },
        ],
    };

    // Serialize the `article` to a JSON string
    let json_str = to_json(&article)?;
    // println!("Serialized JSON: {}", json_str);

    let rust_struct = to_struct(&json_str)?;
    // println!("Deserialized Article: {:?}", rust_struct);

    /*    *****    Parse JSON from file and store in db   *****     */

    let start = Instant::now();

    let path = Path::new("todo.json");

    // Read the JSON file as a string
    let json_data = fs::read_to_string(path)?;

    // Deserialize the JSON string into a `Todo` struct
    let todo_struct: Vec<Todo> = serde_json::from_str(&json_data)?;

    let mut client = create_client()?;

    for (index, todo) in todo_struct.iter().enumerate() {
        insert_todo(&mut client, todo)?;
        println!("{}) {:?}", index + 1, todo);
    }

    let duration = start.elapsed();

    println!("DURATION: {:?}", duration);

    Ok(())
}

fn to_json(article: &Article) -> Result<String, Box<dyn Error>> {
    let json_string = serde_json::to_string(&article)?;
    Ok(json_string)
}

fn to_struct(json_str: &String) -> Result<Article, Box<dyn Error>> {
    let deserialized_string = serde_json::from_str(&json_str)?;

    Ok(deserialized_string)
}

fn insert_todo(client: &mut Client, todo: &Todo) -> Result<(), Box<dyn Error>> {
    client.execute(
        "INSERT INTO todo (user_id, id, title, completed) VALUES($1, $2, $3, $4);",
        &[&todo.userId, &todo.id, &todo.title, &todo.completed],
    )?;
    Ok(())
}

fn create_client() -> Result<Client, Box<dyn Error>> {
    let client = Client::connect(
        "postgres://postgres:postgres@localhost:5433/test_tables",
        NoTls,
    )?;

    Ok(client)
}

/*
{
"userId": 10,
"id": 200,
"title": "ipsam aperiam voluptates qui",
"completed": false
}
*/
