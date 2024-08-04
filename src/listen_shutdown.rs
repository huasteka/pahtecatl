use salvo::server::ServerHandle;
use tokio::signal;

pub async fn listen_shutdown(handle: ServerHandle) {
    // Wait shutdown signal
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Could not install Ctrl+C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Could not install signal handler")
            .recv()
            .await;
    };

    #[cfg(windows)]
    let terminate = async {
        signal::windows::ctrl_c()
          .expect("Could not install signal handler")
          .recv()
          .await;
    };

    tokio::select! {
      _ = ctrl_c => println!("Ctrl+C signal received"),
      _ = terminate => println!("Terminate signal received"),
    }

    handle.stop_graceful(None)
}
