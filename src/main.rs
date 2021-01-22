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
    
}
