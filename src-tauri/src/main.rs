#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use axum::{routing::post, Json};
use serde_json::Value;
use tauri::{AppHandle, Manager};

pub struct AppState {}

#[tauri::command]
fn open_in_browser(url: String) {
    webbrowser::open(&url).ok();
}

#[tauri::command]
fn start_server(app: AppHandle) {
    tauri::async_runtime::spawn(async {
        let addr = "127.0.0.1:1234".parse().unwrap();

        let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);

        let router = axum::Router::new().route(
            "/receive-auth",
            post(|Json(auth_data): Json<Value>| async move {
                println!("Auth received");
                tx.send(()).await.ok();
                app.emit_all("auth_received", auth_data).ok();
                ()
            }),
        );

        println!("Server starting");

        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .with_graceful_shutdown(async move {
                rx.recv().await;
            })
            .await
            .unwrap();

        println!("Shutting down");
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_in_browser, start_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
