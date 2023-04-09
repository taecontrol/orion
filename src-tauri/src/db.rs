use std::fs;
use std::path::Path;
use serde::Deserialize;
use surrealdb::{dbs, Surreal};
use surrealdb::kvs::Datastore;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn init() -> Result<(), E> {
    if !db_file_exists() {
        create_db_file();
    }

    let ds = Datastore::new("file://" + &get_db_path()).await?;
    let db = Surreal::new::<dbs::Kvs>(ds).await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("orion").use_db("orion").await?;

    let created: Record = db
        .create("person")
        .content(Person {
            name: "John Doe".to_string(),
        })
        .await?;

    dbg!(created);

    Ok(())
}

fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    fs::File::create(db_path).unwrap();
}

fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    return home_dir.to_str().unwrap().to_string() + "/.config/orion/database.db";
}
