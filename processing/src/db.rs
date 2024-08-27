use std::{env, error::Error};
use log::{info, debug};
use r2d2_mysql::mysql::{OptsBuilder, Opts};
use r2d2::Pool;
use r2d2_mysql::MysqlConnectionManager;

pub struct DatabaseConfig {
    pub db_pool: Pool<MysqlConnectionManager>,
}

impl DatabaseConfig {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let params = Opts::from_url(&database_url)?;
        info!("Connecting to database: {}", database_url);
        debug!("Params: {:?}", params);

        let ip_or_hostname = params.get_ip_or_hostname().unwrap();
        debug!("ip_or_hostname: {}", ip_or_hostname);

        let db_name = params.get_db_name().unwrap();
        debug!("db_name: {}", db_name);

        let user = params.get_user().unwrap();
        let pass = params.get_pass().unwrap();

        let mut builder = OptsBuilder::new();
        builder.ip_or_hostname(Some(ip_or_hostname)).db_name(Some(db_name)).user(Some(user)).pass(Some(pass));

        let manager = MysqlConnectionManager::new(builder);
        let db_pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        Ok(DatabaseConfig { db_pool })
    }
}
