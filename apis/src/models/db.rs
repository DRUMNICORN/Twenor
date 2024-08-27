use r2d2_mysql::mysql::{OptsBuilder, Opts};
use r2d2::Pool;
use r2d2_mysql::MysqlConnectionManager;
use std::env;

pub struct Db {
    pub pool: Pool<MysqlConnectionManager>,
}

impl Db {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let params = Opts::from_url(&database_url).unwrap();
        let ip_or_hostname = params.get_ip_or_hostname().unwrap();
        let db_name = params.get_db_name().unwrap();

        let user = params.get_user().unwrap();
        let pass = params.get_pass().unwrap();

        let mut builder = OptsBuilder::new();
        
        builder.ip_or_hostname(Some(ip_or_hostname)).db_name(Some(db_name)).user(Some(user)).pass(Some(pass));


        let manager = MysqlConnectionManager::new(builder);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        Db { pool }
    }

}
