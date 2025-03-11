use mini_redis::Result;
use surrealdb::{
    engine::{
        any,
        remote::ws::{Client, Ws},
    },
    opt::auth::Root,
    Surreal,
};

#[tokio::main]
async fn main() -> Result<()> {
    // let db = any::connect("ws://db:8000").await.unwrap();
    let db = Surreal::new::<Ws>("ws://db:8000").await.unwrap();
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    db.use_ns("dev_1").use_db("dev_1").await.unwrap();

    let result: Option<surrealdb::sql::Thing> = db
        .create("koook")
        .content(SomeData {
            a: 1,
            b: "hello".to_string(),
        })
        .await
        .unwrap();
    println!("{:?}", result);
    Ok(())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct SomeData {
    a: i32,
    b: String,
}
