use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Candle {
    time: &'static str,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

#[derive(Serialize)]
struct Summary {
    pair: &'static str,
    timeframe: &'static str,
    sample_note: &'static str,
    last_close: f64,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/api/ohlc", get(ohlc))
        .route("/api/summary", get(summary));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Forex starter server running on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind address");
    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn index() -> &'static str {
    INDEX_HTML
}

async fn ohlc() -> Json<Vec<Candle>> {
    Json(sample_candles())
}

async fn summary() -> Json<Summary> {
    let candles = sample_candles();
    let last_close = candles.last().map(|c| c.close).unwrap_or(0.0);
    Json(Summary {
        pair: "EUR/USD",
        timeframe: "Daily",
        sample_note: "Synthetic data for learning",
        last_close,
    })
}

fn sample_candles() -> Vec<Candle> {
    vec![
        Candle {
            time: "2024-10-01",
            open: 1.0750,
            high: 1.0830,
            low: 1.0710,
            close: 1.0805,
        },
        Candle {
            time: "2024-10-02",
            open: 1.0805,
            high: 1.0860,
            low: 1.0780,
            close: 1.0822,
        },
        Candle {
            time: "2024-10-03",
            open: 1.0822,
            high: 1.0895,
            low: 1.0810,
            close: 1.0880,
        },
        Candle {
            time: "2024-10-04",
            open: 1.0880,
            high: 1.0900,
            low: 1.0825,
            close: 1.0840,
        },
        Candle {
            time: "2024-10-07",
            open: 1.0840,
            high: 1.0870,
            low: 1.0795,
            close: 1.0812,
        },
        Candle {
            time: "2024-10-08",
            open: 1.0812,
            high: 1.0855,
            low: 1.0788,
            close: 1.0835,
        },
        Candle {
            time: "2024-10-09",
            open: 1.0835,
            high: 1.0910,
            low: 1.0820,
            close: 1.0892,
        },
    ]
}

const INDEX_HTML: &str = r#"<!doctype html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
    <title>Forex Starter Dashboard</title>
    <style>
      :root {
        color-scheme: light;
        font-family: "Inter", "Segoe UI", system-ui, sans-serif;
      }
      body {
        margin: 0;
        background: #f7f9fc;
        color: #102a43;
      }
      header {
        padding: 24px 32px 12px;
        background: #ffffff;
        box-shadow: 0 1px 2px rgba(16, 42, 67, 0.08);
      }
      main {
        padding: 24px 32px 40px;
        max-width: 1100px;
        margin: 0 auto;
      }
      .grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
        gap: 16px;
      }
      .card {
        background: #fff;
        border-radius: 12px;
        padding: 16px 20px;
        box-shadow: 0 10px 20px rgba(16, 42, 67, 0.08);
      }
      .card h3 {
        margin: 0 0 8px;
        font-size: 16px;
        color: #3e4c59;
      }
      #chart-wrap {
        padding: 16px;
      }
      canvas {
        width: 100% !important;
        height: 320px !important;
      }
      footer {
        margin-top: 24px;
        color: #52616b;
      }
      .pill {
        display: inline-block;
        padding: 4px 10px;
        border-radius: 999px;
        background: #e3f2fd;
        color: #1e3a8a;
        font-size: 12px;
        font-weight: 600;
        margin-right: 8px;
      }
    </style>
  </head>
  <body>
    <header>
      <h1>Forex Starter Dashboard</h1>
      <p>Learn Rust + web basics with a tiny forex analytics playground.</p>
    </header>
    <main>
      <section class=\"grid\">
        <div class=\"card\">
          <h3>Market Pair</h3>
          <div id=\"pair\">Loading...</div>
        </div>
        <div class=\"card\">
          <h3>Timeframe</h3>
          <div id=\"timeframe\">Loading...</div>
        </div>
        <div class=\"card\">
          <h3>Latest Close</h3>
          <div id=\"last-close\">Loading...</div>
        </div>
      </section>

      <section class=\"card\" id=\"chart-wrap\">
        <h3>Price Trend (Line View)</h3>
        <canvas id=\"priceChart\"></canvas>
        <footer>
          <span class=\"pill\">Demo data</span>
          <span>Built with Rust (Axum) + Chart.js.</span>
        </footer>
      </section>
    </main>

    <script src=\"https://cdn.jsdelivr.net/npm/chart.js\"></script>
    <script>
      async function loadSummary() {
        const response = await fetch('/api/summary');
        const data = await response.json();
        document.getElementById('pair').textContent = data.pair;
        document.getElementById('timeframe').textContent = data.timeframe;
        document.getElementById('last-close').textContent = data.last_close.toFixed(4);
      }

      async function loadChart() {
        const response = await fetch('/api/ohlc');
        const candles = await response.json();
        const labels = candles.map(c => c.time);
        const closes = candles.map(c => c.close);

        const ctx = document.getElementById('priceChart').getContext('2d');
        new Chart(ctx, {
          type: 'line',
          data: {
            labels,
            datasets: [{
              label: 'EUR/USD Close',
              data: closes,
              borderColor: '#2563eb',
              backgroundColor: 'rgba(37, 99, 235, 0.2)',
              tension: 0.25,
              fill: true,
            }]
          },
          options: {
            responsive: true,
            scales: {
              y: {
                title: {
                  display: true,
                  text: 'Price'
                }
              }
            }
          }
        });
      }

      loadSummary();
      loadChart();
    </script>
  </body>
</html>"#;
