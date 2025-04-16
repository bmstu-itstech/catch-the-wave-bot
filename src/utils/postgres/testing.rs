use std::path::{Path, PathBuf};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use crate::utils::postgres::migrations::golang_migrate;


const TEST_DATABASE_URI: &str = "postgres://cw:s3cr3tpw@localhost:54330/cw?sslmode=disable";

fn migrations_uri() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("migrations")
}

pub async fn test_db_setup() -> Pool {
    let mut cfg = Config::new();
    cfg.url = Some(
        TEST_DATABASE_URI
            .parse()
            .expect("failed to parse database connection string")
    );

    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("failed to create pool");

    let exit_code = golang_migrate(TEST_DATABASE_URI, &migrations_uri()).await;
    assert!(exit_code.success());

    pool
}
