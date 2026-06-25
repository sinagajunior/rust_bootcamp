use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;


#[derive(Deserialize)]
struct SignupRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize,Clone)]
struct User {
    username: String,
    hashed_password: String,
}

struct AppState {
    users: Mutex<HashMap<String, User>>,
}

async fn signup(req: web::Json<SignupRequest>, state: web::Data<AppState>) -> impl Responder {
    let mut users = state.users.lock().unwrap();
    let username = &req.username;
    if users.contains_key(username) {
        return HttpResponse::Conflict().body("Username already exists");
    }
    let hashed_password = hash(&req.password, DEFAULT_COST).unwrap();
    let user = User {
        username: req.username.clone(),
        hashed_password,
    };
    users.insert(username.clone(), user);
    HttpResponse::Created().body("User registered successfully")
}


async fn login(req: web::Json<LoginRequest>, state: web::Data<AppState>) -> impl Responder {
    let users = state.users.lock().unwrap();
    match users.get(&req.username) {
    Some(user) =>{
            if verify(&req.password, &user.hashed_password).unwrap_or(false) {
                HttpResponse::Ok().body("Login successful")
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        None => HttpResponse::NotFound().body("User not found"),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Auth API running on http://127.0.0.1:8080");

    let users = web::Data::new(AppState{
        users: Mutex::new(HashMap::new()),
    });

   HttpServer::new(move||{
       App::new()
           .app_data(users.clone())
           .route("/signup", web::post().to(signup))
           .route("/login", web::post().to(login))
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}
