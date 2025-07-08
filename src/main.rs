use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use clap::Parser;
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;

/// 定義命令列參數
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Power Automate Webhook 的 URL
    #[arg(long)]
    webhook_url: String,

    /// 綁定的 IP 位址
    #[arg(long, default_value = "0.0.0.0")]
    bind_ip: String,

    /// 綁定的埠號
    #[arg(long, default_value_t = 3080)]
    bind_port: u16,
}

/// 應用程式狀態，儲存 Webhook URL 和 HTTP 客戶端
struct AppState {
    webhook_url: String,
    client: Client,
}

/// 處理傳入的 Grafana Webhook 並轉發到 Power Automate
#[post("/grafana-webhook")]
async fn forward_alert(
    data: web::Data<Arc<AppState>>,
    payload: web::Json<Value>,
) -> impl Responder {
    let client = &data.client;
    let webhook_url = &data.webhook_url;

    println!("Received a request. Payload: {:?}", payload);

    // 將收到的 payload 轉發到指定的 Webhook URL
    println!("Forwarding to: {}", webhook_url);
    match client
        .post(webhook_url)
        .json(&payload.into_inner())
        .send()
        .await
    {
        Ok(response) => {
            // 從 reqwest 的狀態碼轉換為 actix_web 的狀態碼
            let status = actix_web::http::StatusCode::from_u16(response.status().as_u16()).unwrap();
            let text = response.text().await.unwrap_or_default();
            println!(
                "Received response from webhook. Status: {}. Body: {}",
                status, text
            );
            // 回傳從目標伺服器收到的回應
            HttpResponse::build(status).body(text)
        }
        Err(e) => {
            // 如果轉發失敗，記錄錯誤並回傳 500 錯誤
            eprintln!("Forwarding error: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 解析命令列參數
    let args = Args::parse();

    // 建立一個 reqwest 客戶端，並設定為接受無效的 SSL 憑證
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to build HTTP client");

    // 建立應用程式狀態
    let state = Arc::new(AppState {
        webhook_url: args.webhook_url,
        client,
    });

    println!(
        "Starting Grafana proxy on http://{}:{}",
        args.bind_ip, args.bind_port
    );

    // 啟動 HTTP 伺服器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(forward_alert)
    })
    .bind((args.bind_ip.clone(), args.bind_port))?
    .run()
    .await
}
