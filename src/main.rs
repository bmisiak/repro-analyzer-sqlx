struct Model {
    id: i32
}

 fn repro(postgres: &sqlx::PgPool) -> () {
    let _x = sqlx::query_as::<_,Model>("SELECT 1")
    .bind(12) // comment this out
    .fetch_one(postgres);

    ()
}

fn main() {
    
}
