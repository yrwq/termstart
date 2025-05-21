use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;
use validator::Validate;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct RegisterRequest {
    #[validate(email)]
    email: String,
    #[validate(length(min = 3, max = 50))]
    username: String,
    #[validate(length(min = 8))]
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    username: String,
    email: String,
    token: String,
    is_authenticated: bool,
}

async fn register(
    pool: web::Data<sqlx::PgPool>,
    register: web::Json<RegisterRequest>
) -> impl Responder {
    // Validate request
    if let Err(errors) = register.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    // Check if email already exists
    let email_exists = sqlx::query!(
        "SELECT COUNT(*) as count FROM users WHERE email = $1",
        register.email
    )
    .fetch_one(&**pool)
    .await;

    match email_exists {
        Ok(result) if result.count.unwrap_or(0) > 0 => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Email already registered"
            }));
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }));
        }
        _ => {}
    }

    // Hash password
    let hashed_password = match bcrypt::hash(register.password.as_bytes(), bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Password hashing failed"
            }));
        }
    };

    // Insert new user
    let result = sqlx::query!(
        "INSERT INTO users (email, username, password_hash) VALUES ($1, $2, $3) RETURNING id",
        register.email,
        register.username,
        hashed_password
    )
    .fetch_one(&**pool)
    .await;

    match result {
        Ok(_) => {
            let token = generate_token(&register.email);
            HttpResponse::Ok().json(LoginResponse {
                username: register.username.clone(),
                email: register.email.clone(),
                token,
                is_authenticated: true,  // Add this field
            })
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create user"
            }))
        }
    }
}

async fn login(
    pool: web::Data<sqlx::PgPool>,
    login: web::Json<LoginRequest>
) -> impl Responder {
    // Find user by email
    let user = sqlx::query!(
        "SELECT id, email, username, password_hash FROM users WHERE email = $1",
        login.email
    )
    .fetch_optional(&**pool)
    .await;

    match user {
        Ok(Some(user)) => {
            match bcrypt::verify(login.password.as_bytes(), &user.password_hash) {
                Ok(true) => {
                    let token = generate_token(&user.email);
                    let response = LoginResponse {
                        username: user.username,
                        email: user.email,
                        token,
                        is_authenticated: true,  // Add this field
                    };
                    HttpResponse::Ok().json(response)
                }
                _ => HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Invalid credentials"
                }))
            }
        }
        Ok(None) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid credentials"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Database error"
        }))
    }
}

fn generate_token(email: &str) -> String {
    // This should return a JWT token string
    format!("dummy_token_{}", email) // Replace with actual JWT implementation
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
            
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/api/register")
                    .route(web::post().to(register))
            )
            .service(
                web::resource("/api/login")
                    .route(web::post().to(login))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}