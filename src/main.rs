mod app;
use app::app::app;
use std::{env, error::Error};
use tokio::signal;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = app().await;
    let address = format!(
        "{}:{}",
        "0.0.0.0",
        env::var("PORT").unwrap_or("8000".to_string())
    );
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    Ok(())
}

async fn shutdown_signal() {
    // Wait for either SIGINT (Ctrl+C) or SIGTERM (docker stop)
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};
        let mut term = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        term.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Signal received, starting graceful shutdown...");
}
