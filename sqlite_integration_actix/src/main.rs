use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::{SqlitePool, FromRow};
use serde::{Serialize, Deserialize};


#[derive(Serialize, FromRow)]
struct Post {
    id: i64,
    title: String,
    content: String,
}

#[derive(Deserialize)]
struct NewPost {
    title: String,
    content: String,
}


async fn get_post(db: web::Data<SqlitePool>) -> impl Responder {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(db.get_ref())
        .await;

    match posts{
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB Error  :  {}", e)),
    }

}

async fn add_post(db: web::Data<SqlitePool>,
    json: web::Json<NewPost>,
) -> impl Responder {
    let result = sqlx::query("INSERT INTO posts (title,content) values (?,?)")
        .bind(&json.title)
        .bind(&json.content)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Post added successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Insert Failed  :  {}", e)),
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started at http://localhost:8080");

    let db = SqlitePool::connect("sqlite:blog.db").await.expect("DB Connect failed");

    HttpServer::new(move || App::new()
        .app_data(web::Data::new(db.clone()))
        .route("/posts", web::get().to(get_post))
        .route("/posts", web::post().to(add_post))
    )
    .bind("localhost:8080")?
    .run()
    .await?;

    Ok(())
}
