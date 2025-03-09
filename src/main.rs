//****COIN PULSE****
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection, Result};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_files;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Deserialize, Debug, Serialize)]
struct PriceInfo {

    usd: f64,
    usd_24h_change: Option<f64>,
    last_updated_at: Option<u64>,
}

#[derive(Clone)]
struct AppState {

    coin_ids: Arc<Vec<String>>,
}

fn timestamp(timestamp: Option<u64>) -> String {

    if let Some(ts) = timestamp {

        if let Some(dt) = chrono::DateTime::from_timestamp(ts as i64, 0) {

            return dt.format("%Y-%m-%d %H:%M:%S").to_string();
        }
    }
    "Unknown".to_string()
}

async fn get_coin_data(coin_ids: &Vec<String>) -> Result<HashMap<String, PriceInfo>, anyhow::Error> {

    let ids  = coin_ids.join(",");
    let currency = "usd";
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}&include_24hr_change=true&include_last_updated_at=true",
        ids, currency
    );

    let response = reqwest::get(&url).await?;

    let text = response.text().await?;
    eprintln!("CoinGecko API response: {}", text);

    let data: HashMap<String, PriceInfo> =  serde_json::from_str(&text)?;

    Ok(data)
}

#[get("/api/crypto")]
async fn fetch_coin_data(data: web::Data<AppState>) -> impl Responder {

    let coin_ids= &data.coin_ids;

    match get_coin_data(coin_ids).await {

        Ok(data ) => {

            let formatted_data: HashMap<String, serde_json::Value> = data
                .into_iter()
                .map(|(name, info)| {
                    let formatted_info = serde_json::json!({
                        "usd": info.usd,
                        "usd_24h_change": info.usd_24h_change,
                        "last_updated_at": timestamp(info.last_updated_at),
                    });
                    (name, formatted_info)
                })
                .collect();

            println!("Fetched data:\n{}", serde_json::to_string_pretty(&formatted_data).unwrap());
            
            HttpResponse::Ok().json(formatted_data)
        },
        Err(e) => {

            eprintln!("Failed to fetch data: {e}");
            HttpResponse::InternalServerError().body("Failed to fetch coin data")
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let coin_ids = vec!["bitcoin".to_string(), "dogecoin".to_string(), "ethereum".to_string()];

    let shared_data = web::Data::new(AppState {

        coin_ids: Arc::new(coin_ids),
    });

    HttpServer::new(move || {

        App::new()
            .app_data(shared_data.clone())
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .route("/", web::get().to(|| async {

                HttpResponse::Found()
                    .append_header(("Location", "static/index.html"))
                    .finish()
            }))
            .service(fetch_coin_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
