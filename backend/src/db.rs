use sqlx::postgres::PgPool;

pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(database_url).await?;
    migrate(&pool).await?;
    seed(&pool).await?;
    Ok(pool)
}

const MIGRATION: &str = r#"
CREATE TABLE IF NOT EXISTS "user" (
    user_id VARCHAR(10) PRIMARY KEY,
    user_name VARCHAR(50) NOT NULL,
    phone VARCHAR(20) NOT NULL
);

CREATE TABLE IF NOT EXISTS item (
    item_id VARCHAR(10) PRIMARY KEY,
    item_name VARCHAR(100) NOT NULL,
    category VARCHAR(50) NOT NULL,
    price INT NOT NULL CHECK (price >= 0),
    status INT NOT NULL DEFAULT 0 CHECK (status IN (0, 1)),
    seller_id VARCHAR(10) NOT NULL REFERENCES "user"(user_id)
);

CREATE TABLE IF NOT EXISTS orders (
    order_id VARCHAR(10) PRIMARY KEY,
    item_id VARCHAR(10) NOT NULL UNIQUE REFERENCES item(item_id),
    buyer_id VARCHAR(10) NOT NULL REFERENCES "user"(user_id),
    order_date DATE NOT NULL
);

CREATE OR REPLACE VIEW sold_items_view AS
SELECT item.item_id, item.item_name, item.price, item.category,
       orders.order_id, orders.buyer_id, u.user_name AS buyer_name, orders.order_date
FROM item
JOIN orders ON item.item_id = orders.item_id
JOIN "user" u ON orders.buyer_id = u.user_id
WHERE item.status = 1
ORDER BY item.item_id;

CREATE OR REPLACE VIEW unsold_items_view AS
SELECT item_id, item_name, category, price, seller_id
FROM item
WHERE status = 0
ORDER BY item_id;
"#;

fn split_sql(sql: &str) -> Vec<&str> {
    sql.split(';').map(str::trim).filter(|s| !s.is_empty()).collect()
}

async fn migrate(pool: &PgPool) -> Result<(), sqlx::Error> {
    for stmt in split_sql(MIGRATION) {
        sqlx::query(stmt).execute(pool).await?;
    }
    Ok(())
}

const SEED: &str = r#"
INSERT INTO "user" (user_id, user_name, phone) VALUES
    ('u001', 'ZhangSan', '13800000001'),
    ('u002', 'LiSi', '13800000002'),
    ('u003', 'WangWu', '13800000003'),
    ('u004', 'ZhaoLiu', '13800000004')
ON CONFLICT (user_id) DO NOTHING;

INSERT INTO item (item_id, item_name, category, price, status, seller_id) VALUES
    ('i001', 'CalculusBook', 'Book', 20, 0, 'u001'),
    ('i002', 'DeskLamp', 'DailyGoods', 35, 1, 'u002'),
    ('i003', 'Microcontroller', 'Electronics', 80, 0, 'u001'),
    ('i004', 'Chair', 'Furniture', 50, 1, 'u003'),
    ('i005', 'WaterBottle', 'DailyGoods', 15, 0, 'u004')
ON CONFLICT (item_id) DO NOTHING;

INSERT INTO orders (order_id, item_id, buyer_id, order_date) VALUES
    ('o001', 'i002', 'u001', '2024-05-01'),
    ('o002', 'i004', 'u002', '2024-05-03')
ON CONFLICT (order_id) DO NOTHING;
"#;

async fn seed(pool: &PgPool) -> Result<(), sqlx::Error> {
    for stmt in split_sql(SEED) {
        sqlx::query(stmt).execute(pool).await?;
    }
    Ok(())
}

pub async fn reset(pool: &PgPool) {
    // truncate in correct FK order
    sqlx::query("DELETE FROM orders").execute(pool).await.expect("Reset: delete orders");
    sqlx::query("DELETE FROM item").execute(pool).await.expect("Reset: delete items");
    sqlx::query("DELETE FROM \"user\"").execute(pool).await.expect("Reset: delete users");
    // re-seed
    for stmt in split_sql(SEED) {
        sqlx::query(stmt).execute(pool).await.expect("Reset: seed failed");
    }
}
