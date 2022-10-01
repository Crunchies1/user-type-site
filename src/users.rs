use actix_web::http::StatusCode;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::output::{PutItemOutput, GetItemOutput, DeleteItemOutput};
use aws_sdk_dynamodb::{Client};
use actix_web::web::{Path};
use actix_web::{get, post, delete, web, HttpResponse, Responder};

mod actions;

/// GET /users : List the last 20 users.
#[get("/users")]
pub async fn list() -> impl Responder {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let users = actions::list_items(&client, "users").await;
    match users {
        Ok(_) => HttpResponse::Ok().body(format!("{:?}", users)),
        Err(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e.to_string())
    }
}

/// POST /users : Create a user.
#[post("/users")]
pub async fn create(json: web::Json<actions::Item>) -> impl Responder {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let item = actions::Item{ 
        id: json.id.to_string(),
        username: json.username.to_string(), 
        password: json.password.to_string(), 
        usertype: json.usertype.to_string()
    };
    let put = actions::add_item(&client, item, "users").await;

    match put {
        Ok(PutItemOutput { attributes, .. }) => HttpResponse::Ok().body(format!("{:?}", attributes)),
        Err(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e.to_string())
    }
}

/// GET /users{id} : Finds a user by their ID.

#[get("/users/{id}")]
pub async fn get(path: Path<(String,)>) -> impl Responder {
    // Create AWS client
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Find user
    let id = path.into_inner().0;
    let found_user = actions::get_item(&client, "users", "id", &id).await;

    match found_user {
        Ok(GetItemOutput { item, .. }) => HttpResponse::Ok().body(format!("{:?}", item)),
        Err(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e.to_string())
    }
}


/// DELETE /users{id} : Delete a user by their ID.
#[delete("/users/{id}")]
pub async fn delete(path: Path<(String,)>) -> impl Responder {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let id = path.into_inner().0;
    let delete = actions::delete_item(&client, "users", "id", &id).await;

    match delete {
        Ok(DeleteItemOutput { attributes, .. }) => HttpResponse::Ok().body(format!("{:?}", attributes)),
        Err(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e.to_string())
    }
}