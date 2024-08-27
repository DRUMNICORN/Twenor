use r2d2::{Pool, PooledConnection};
use r2d2_mysql::{
    mysql::{Opts, OptsBuilder, SslOpts}, MySqlConnectionManager,
};
use std::{env, error::Error, result::Result};

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    db_pool: Pool<MySqlConnectionManager>,
}
impl DatabaseConfig {
    pub fn new() -> std::result::Result<Self, Box<dyn Error>> {
        let database_url = env::var("DATABASE_URL")?;
        let params = Opts::from_url(&database_url)?;
        let binding = params.get_ip_or_hostname().clone().into_owned();
        let ip_or_hostname: &str = binding.as_str();
        let db_name = params.get_db_name().ok_or_else(|| "Database name is required")?;
        let user = params.get_user().ok_or_else(|| "User is required")?;
        let pass = params.get_pass().ok_or_else(|| "Password is required")?;
        let cert_path = std::path::Path::new("DANK-ca-certificate.crt");
        if !cert_path.exists() {
            return Err("DANK-ca-certificate.crt not found".into());
        }
        log::info!("Connecting to database: {}", ip_or_hostname);
        let manager = MySqlConnectionManager::new(OptsBuilder::new()
        .ip_or_hostname(Some(ip_or_hostname))
        .db_name(Some(db_name))
        .user(Some(user))
        .pass(Some(pass))
        .ssl_opts(Some(SslOpts::default().with_root_cert_path(Some(cert_path)))));

        let db_pool = Pool::builder()
            .build(manager)?;
        
        Ok(DatabaseConfig { db_pool })
        
    }

    pub fn get_db_pool(&self) -> Pool<MySqlConnectionManager> {
        self.db_pool.clone()
    }

    pub fn get_connection(
        &self,

    ) -> Result<PooledConnection<MySqlConnectionManager>, Box<dyn Error>> {
        match self.db_pool.get() {
            Ok(conn) => Ok(conn),
            Err(err) => Err(Box::new(err)),
        }
    }
}
