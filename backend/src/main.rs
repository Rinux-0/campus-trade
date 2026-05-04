mod db;
mod handlers;
mod models;
mod queries;
mod routes;

use sqlx::PgPool;
use tower_http::{cors::CorsLayer, services::ServeDir};

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

fn find_frontend_dir() -> String {
    if let Ok(dir) = std::env::var("FRONTEND_DIR") {
        return dir;
    }
    // Resolve relative to the executable location, so it works regardless of CWD.
    if let Ok(exe) = std::env::current_exe() {
        let mut path = exe.canonicalize().unwrap_or(exe);
        for _ in 0..5 {
            path.pop(); // walk up
            let candidate = path.join("frontend").join("dist");
            if candidate.is_dir() {
                return candidate.to_string_lossy().into();
            }
        }
    }
    // Fallback to CWD-relative (useful during development via `cargo run`).
    "../frontend/dist".into()
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/campus_trade".into());
    let pool = db::init_pool(&database_url).await.unwrap_or_else(|e| {
        eprintln!(
            "Failed to connect to PostgreSQL: {e}\n\
             Hint: check that PostgreSQL is running and DATABASE_URL is correct.\n\
             Current DATABASE_URL: {url}",
            url = database_url,
        );
        std::process::exit(1);
    });

    let state = AppState { pool };
    let frontend_dir = find_frontend_dir();

    let app = routes::create_router()
        .fallback_service(ServeDir::new(&frontend_dir))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap_or_else(|e| {
            eprintln!(
                "Failed to bind port {port}: {e}\n\
                 Hint: port may be in use. Try `PORT=3001 {exe}`",
                port = port,
                exe = std::env::current_exe()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| "campus-trade".into()),
            );
            std::process::exit(1);
        });
    println!("Server running on http://0.0.0.0:{}", port);
    println!("Swagger UI: http://0.0.0.0:{}/api", port);
    println!("Serving frontend from: {}", frontend_dir);
    axum::serve(listener, app).await.unwrap_or_else(|e| {
        eprintln!("Server error: {e}");
        std::process::exit(1);
    });
}
