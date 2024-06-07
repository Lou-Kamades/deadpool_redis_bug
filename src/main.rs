#[tokio::main]
async fn main() {
    let manager = deadpool_redis::Manager::new("127.0.0.1:6379").unwrap();
    let redis_pool = deadpool_redis::Pool::builder(manager).build().unwrap();
    let mut conn = redis_pool.get().await.unwrap();

    let _: () = deadpool_redis::redis::pipe()
        .set("A", 1)
        .set("B", 2)
        .query(&mut conn)
        .unwrap();
}
