use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    rw_pool: PgPool,
    ro_pool: PgPool,
}

#[derive(Deserialize)]
struct CreateProductRequest {
    name: String,
    description: Option<String>,
    category_id: i32,
    sku: String,
    price: f64,
    attributes: Value,
}

#[derive(Serialize)]
struct ProductResponse {
    id: i32,
    name: String,
    price: String,
    quantity: i32,
    attributes: Value,
}

#[tokio::main]
async fn main() {
    let user = "app";
    let pass = "3SIJeb95dn1YSyEwEsa8zvJXDgPrbkjFEKx5oBQZnA4wSSZTeMhVJG3KQgD39Bdv";
    let host_base = "localhost";
    let db_name = "app";
    let rw_port = "5432";
    let ro_port = "5432";
    
    let rw_url = format!("postgres://{user}:{pass}@{host_base}:{rw_port}/{db_name}");
    let ro_url = format!("postgres://{user}:{pass}@{host_base}:{ro_port}/{db_name}");

    println!("Connecting to RW DB at: {rw_url}");

    let rw_pool = PgPool::connect(&rw_url).await.expect("Failed to connect to Master (RW)");
    let ro_pool = PgPool::connect(&ro_url).await.expect("Failed to connect to Replica (RO)");

    let state = AppState { rw_pool, ro_pool };

    let app = Router::new()
        .route("/products", get(list_products).post(add_product))
        .route(
            "/products/:id", 
            get(get_product)
            .put(update_product) // Standard practice: use PUT for updates
            .delete(delete_product)
        )
        .with_state(state);

    println!("🚀 Server running on http://0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn list_products(State(state): State<AppState>) -> Json<Value> {
    let rows = sqlx::query!(
        r#"
        SELECT  p.id, p.name, pv.price, pv.stock_quantity, pv.attributes 
        FROM products p 
        JOIN product_variants pv ON p.id = pv.product_id
        "#
    )
    .fetch_all(&state.ro_pool)
    .await
    .unwrap();

    let list: Vec<ProductResponse> = rows
        .into_iter()
        .map(|row| ProductResponse {
            id: row.id,
            name: row.name,
            price: row.price.to_string(),
            quantity: row.stock_quantity.unwrap_or(0),
            attributes: row.attributes,
        })
        .collect();

    Json(json!(list))
}

async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ProductResponse>, StatusCode> {
    let row = sqlx::query!(
        r#"
        SELECT  p.id, p.name, pv.price, pv.stock_quantity, pv.attributes 
        FROM products p 
        JOIN product_variants pv ON p.id = pv.product_id
        WHERE p.id = $1
        "#,
        id
    )
    .fetch_optional(&state.ro_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(row) => Ok(Json(ProductResponse {
            id: row.id,
            name: row.name,
            price: row.price.to_string(),
            quantity: row.stock_quantity.unwrap_or(0),
            attributes: row.attributes,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn add_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    let mut tx = state.rw_pool.begin().await.unwrap();

    // Insert to products
    let product = sqlx::query!(
        "INSERT INTO products (name, description, category_id) VALUES ($1, $2, $3) RETURNING id",
        payload.name,
        payload.description,
        payload.category_id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insert to variants
    let res = sqlx::query!(
        "INSERT INTO product_variants (product_id, sku, price, attributes) VALUES ($1, $2, $3, $4)",
        product.id,
        payload.sku,
        payload.price as f32,
        payload.attributes
    )
    .execute(&mut *tx)
    .await;

    match res {
        Ok(_) => {
            tx.commit().await.unwrap();
            // Return 201 Created and the ID
            Ok((StatusCode::CREATED, Json(json!({ "id": product.id }))))
        }
        Err(_) => {
            let _ = tx.rollback().await;
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<Value>,
) -> StatusCode {
    let new_name = payload["name"].as_str().unwrap_or("Updated Product");
    
    let res = sqlx::query!("UPDATE products SET name = $1 WHERE id = $2", new_name, id)
        .execute(&state.rw_pool)
        .await;

    match res {
        Ok(r) if r.rows_affected() > 0 => StatusCode::OK,
        _ => StatusCode::NOT_FOUND,
    }
}

async fn delete_product(State(state): State<AppState>, Path(id): Path<i32>) -> StatusCode {
    let res = sqlx::query!("DELETE FROM products WHERE id = $1", id)
        .execute(&state.rw_pool)
        .await;

    match res {
        Ok(r) if r.rows_affected() > 0 => StatusCode::NO_CONTENT,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}