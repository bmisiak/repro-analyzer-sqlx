#[derive(sqlx::FromRow)]
struct Model {
    id: i32
}

async fn repro(postgres: &sqlx::PgPool) -> Result<Model, sqlx::Error> {
    let x = sqlx::query_as::<_,Model>("SELECT 1")
    .bind(12) // comment this out
    .fetch_one(postgres);

    x.await
}

fn main() {
    async_std::task::block_on(async {
        let pool = sqlx::postgres::PgPool::connect("postgres://localhost/repro").await.unwrap();
        let model = repro(&pool).await.unwrap();
        println!("{}",model.id);
    });
}
