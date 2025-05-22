use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder, middleware};
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
    is_admin: bool,
}

#[derive(Serialize)]
struct User {
    username: String,
    email: String,
    is_admin: bool,
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
                is_authenticated: true,
                is_admin: false,
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
        "SELECT id, email, username, password_hash, is_admin FROM users WHERE email = $1",
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
                        is_authenticated: true,
                        is_admin: user.is_admin.unwrap_or(false),
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

async fn list_users(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
) -> impl Responder {
    let token = req.headers().get("Authorization").and_then(|h| h.to_str().ok()).unwrap_or("");
    
    // Validate admin token
    let admin = sqlx::query!(
        "SELECT is_admin FROM users WHERE email = $1",
        token.replace("dummy_token_", "")
    )
    .fetch_optional(&**pool)
    .await;

    match admin {
        Ok(Some(user)) if user.is_admin.unwrap_or(false) => {
            let users = sqlx::query!(
                "SELECT username, email, is_admin FROM users"
            )
            .fetch_all(&**pool)
            .await;

            match users {
                Ok(users) => {
                    let users: Vec<User> = users.into_iter()
                        .map(|u| User {
                            username: u.username,
                            email: u.email,
                            is_admin: u.is_admin.unwrap_or(false),
                        })
                        .collect();
                    HttpResponse::Ok().json(users)
                }
                Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to fetch users"
                }))
            }
        }
        _ => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized: Admin access required"
        }))
    }
}

async fn debug_info(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
) -> impl Responder {
    let token = req.headers().get("Authorization").and_then(|h| h.to_str().ok()).unwrap_or("");
    
    // Validate admin token
    let admin = sqlx::query!(
        "SELECT is_admin FROM users WHERE email = $1",
        token.replace("dummy_token_", "")
    )
    .fetch_optional(&**pool)
    .await;

    match admin {
        Ok(Some(user)) if user.is_admin.unwrap_or(false) => {
            let stats = sqlx::query!(
                "SELECT 
                    COUNT(*) as total_users,
                    COUNT(*) FILTER (WHERE is_admin = true) as admin_users,
                    COUNT(*) FILTER (WHERE created_at > NOW() - INTERVAL '24 hours') as new_users_24h
                FROM users"
            )
            .fetch_one(&**pool)
            .await;

            match stats {
                Ok(stats) => {
                    HttpResponse::Ok().json(serde_json::json!({
                        "total_users": stats.total_users,
                        "admin_users": stats.admin_users,
                        "new_users_24h": stats.new_users_24h,
                        "database_url": env::var("DATABASE_URL").unwrap_or_default(),
                        "server_time": chrono::Utc::now().to_rfc3339(),
                    }))
                }
                Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to fetch debug info"
                }))
            }
        }
        _ => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized: Admin access required"
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
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/api/register")
                    .route(web::post().to(register))
            )
            .service(
                web::resource("/api/login")
                    .route(web::post().to(login))
            )
            .service(
                web::resource("/api/admin/users")
                    .route(web::get().to(list_users))
            )
            .service(
                web::resource("/api/admin/debug")
                    .route(web::get().to(debug_info))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}