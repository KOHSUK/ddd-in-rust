use actix_web::{get, post, delete, put, web, App, HttpResponse, HttpServer, Responder};
use std::io;
use serde::{ Serialize, Deserialize };

use crate::interface::controller::user_controller::{UserController, GetArgs, PostArgs, DeleteArgs, PutArgs};

pub struct WebServer;

impl WebServer {
    pub async fn run(&self) -> io::Result<()> {
        HttpServer::new(move || {
            App::new()
                .service(get_user)
                .service(post_user)
                .service(post_user)
                .service(delete_user)
                .service(put_user)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }

    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize)]
struct GetUserResult {
    id: String,
    name: String,
}

#[get("/user/{id}")]
async fn get_user(path: web::Path<(String,)>) -> impl Responder {
    let id = path.into_inner().0;

    let args = GetArgs { id };
    if let Ok(controller) = UserController::new().await {
        match controller.get(args).await {
            Some(u) => {
                let result = GetUserResult { id: u.id, name: u.name };
                HttpResponse::Ok().body(serde_json::to_string(&result).unwrap())
            },
            None => HttpResponse::NotFound().body("Not Found"),
        }
    } else {
        HttpResponse::InternalServerError().body("Internal Server Error")
    }
}

#[derive(Deserialize)]
struct PostUserPayload {
    name: String,
}

#[post("/user")]
async fn post_user(body: web::Json<PostUserPayload>) -> impl Responder {
    if let Ok(controller) = UserController::new().await {
        let args = PostArgs { name: body.name.to_owned() };
        match controller.post(args).await {
            Ok(_) => {
                HttpResponse::Ok().body("OK")
            },
            Err(e) => HttpResponse::NotAcceptable().body(e.to_string())
        }
    } else {
        HttpResponse::InternalServerError().body("Internal Sever Error")
    }
}

#[delete("/user/{id}")]
async fn delete_user(path: web::Path<(String,)>) -> impl Responder {
    let id = path.into_inner().0;

    let args = DeleteArgs { id };
    if let Ok(controller) = UserController::new().await {
        match controller.delete(args).await {
            Ok(_) => HttpResponse::Ok().body("OK"),
            Err(e) => HttpResponse::NotFound().body(e.to_string()),
        }
    } else {
        HttpResponse::InternalServerError().body("Internal Server Error")
    }
}

#[derive(Deserialize)]
struct PutUserPayload {
    id: String,
    name: String,
}

#[put("/user")]
async fn put_user(body: web::Json<PutUserPayload>) -> impl Responder {
    if let Ok(controller) = UserController::new().await {
        let args = PutArgs { id: body.id.to_owned() , name: body.name.to_owned() };
        match controller.put(args).await {
            Ok(_) => HttpResponse::Ok().body("OK"),
            Err(e) => HttpResponse::NotFound().body(e.to_string()),
        }
    } else {
        HttpResponse::InternalServerError().body("Internal Server Error")
    }
}