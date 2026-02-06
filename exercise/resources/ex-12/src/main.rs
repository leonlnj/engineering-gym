use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use redis::{Client};
use serde::Deserialize;
use std::env;
use std::sync::Arc;

#[derive(Debug, Deserialize, Clone, Copy)]
enum Strategy { Fixed, Leaky, Sliding }

#[derive(Clone)]
struct AppConfig {
    redis_url: String,
    strategy: Strategy,
    limit: i64,
    window_secs: i64,
}

struct AppState {
    config: AppConfig,
    redis_client: Client,
}

impl AppState {
    async fn is_allowed(&self, key: &str) -> bool {
        let mut conn = self.redis_client
            .get_multiplexed_async_connection()
            .await
            .expect("Redis connection failed");
        
let (script_str, args) = match self.config.strategy {
            Strategy::Fixed => (
                r#"
                local c = redis.call('INCR', KEYS[1])
                if c == 1 then redis.call('EXPIRE', KEYS[1], ARGV[1]) end
                return c <= tonumber(ARGV[2]) and 1 or 0
                "#,
                (self.config.window_secs, self.config.limit)
            ),
            Strategy::Leaky => (
                r#"
                local time = redis.call('TIME')
                local now = tonumber(time[1]) + (tonumber(time[2]) / 1000000)
                local tat = tonumber(redis.call('GET', KEYS[1])) or now
                
                local emission_interval = ARGV[2] / ARGV[1]
                local new_tat = math.max(tat, now) + emission_interval
                local delay = new_tat - now

                if delay > tonumber(ARGV[2]) then
                    return 0
                else
                    redis.call('SET', KEYS[1], new_tat, 'EX', math.ceil(ARGV[2]))
                    return 1
                end
                "#,
                (self.config.limit, self.config.window_secs)
            ),
            Strategy::Sliding => (
                r#"
                local time = redis.call('TIME')
                local now_secs = tonumber(time[1])
                local now_micros = tonumber(time[2])
                -- Unique member: seconds + micros + random 4-digit number
                local member = now_secs .. "_" .. now_micros .. "_" .. math.random(1000, 9999)
                
                redis.call('ZREMRANGEBYSCORE', KEYS[1], 0, now_secs - ARGV[1])
                if redis.call('ZCARD', KEYS[1]) < tonumber(ARGV[2]) then
                    redis.call('ZADD', KEYS[1], now_secs, member)
                    redis.call('EXPIRE', KEYS[1], ARGV[1])
                    return 1
                else
                    return 0
                end
                "#,
                (self.config.window_secs, self.config.limit)
            ),
        };

        let result: i32 = redis::Script::new(script_str)
            .key(key)
            .arg(args.0)
            .arg(args.1)
            .invoke_async(&mut conn) // Ensure &mut conn is the only arg here
            .await
            .unwrap_or(0);

        result == 1
    }
}

async fn handle_request(
    Path(user_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> StatusCode {
    let key = format!("rate_limit:{user_id}");
    if state.is_allowed(&key).await {
        StatusCode::OK
    } else {
        StatusCode::TOO_MANY_REQUESTS
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Default values if no args are provided
    let strategy_arg = args.get(1).map(|s| s.as_str()).unwrap_or("fixed");
    let limit: i64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(3);
    let window: i64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(10);

    let strategy = match strategy_arg.to_lowercase().as_str() {
        "leaky" => Strategy::Leaky,
        "sliding" => Strategy::Sliding,
        _ => Strategy::Fixed,
    };

    let cfg = AppConfig {
        redis_url: "redis://127.0.0.1/".into(),
        strategy,
        limit,
        window_secs: window,
    };

    println!("Starting with {:?} strategy: {} reqs / {}s", cfg.strategy, cfg.limit, cfg.window_secs);

    let shared_state = Arc::new(AppState {
        redis_client: Client::open(cfg.redis_url.clone()).unwrap(),
        config: cfg,
    });

    let app = Router::new()
        .route("/api/:user_id", get(handle_request))
        .with_state(Arc::clone(&shared_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    println!("Server online | Strategy: {:?}", shared_state.config.strategy);
    
    axum::serve(listener, app).await.unwrap();
}