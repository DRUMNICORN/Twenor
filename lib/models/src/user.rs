// Module defining the User struct

use std::error::Error;

use r2d2_mysql::mysql::prelude::{Queryable, FromRow};
use r2d2_mysql::mysql::{Params, Row};
use serde::{Deserialize, Serialize};

use crate::{Audio, user_authenticated, UserAuthenticated};

use super::DatabaseConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    user_id: i32,
    user_name: String,
    user_access_token: String,
    user_token: String,
    user_email: String,
}

impl FromRow for User {
    fn from_row(row: Row) -> Self {
        let user_id = match row.get("user_id") {
            Some(user_id) => {
                user_id
            }
            None => {
                log::error!("Failed to get user_id");
                0
            }
        };

        let user_name: String = match row.get("user_name") {
            Some(user_name) => {
                user_name
            }
            None => {
                log::error!("Failed to get user_name");
                "".to_string()
            }
        };

        let user_access_token: String = match row.get("user_access_token") {
            Some(user_access_token) => {
                user_access_token
            }
            None => {
                log::error!("Failed to get user_access_token");
                "".to_string()
            }
        };

        let user_token: String = match row.get("user_token") {
            Some(user_token) => {
                user_token
            }
            None => {
                log::error!("Failed to get user_token");
                "".to_string()
            }
        };

        let user_email: String = match row.get("user_email") {
            Some(user_email) => {
                user_email
            }
            None => {
                log::error!("Failed to get user_email");
                "".to_string()
            }
        };

        let user = User {
            user_id,
            user_name,
            user_access_token,
            user_token,
            user_email,
        };
        user
    }

    fn from_row_opt(row: Row) -> Result<Self, r2d2_mysql::mysql::FromRowError>
    where
        Self: Sized {
        let user_id = match row.get("user_id") {
            Some(user_id) => {
                user_id
            }
            None => {
                log::error!("Failed to get user_id");
                0
            }
        };

        let user_name: String = match row.get("user_name") {
            Some(user_name) => {
                user_name
            }
            None => {
                log::error!("Failed to get user_name");
                "".to_string()
            }
        };

        let user_access_token: String = match row.get("user_access_token") {
            Some(user_access_token) => {
                user_access_token
            }
            None => {
                log::error!("Failed to get user_access_token");
                "".to_string()
            }
        };

        let user_token: String = match row.get("user_token") {
            Some(user_token) => {
                user_token
            }
            None => {
                log::error!("Failed to get user_token");
                "".to_string()
            }
        };

        let user_email: String = match row.get("user_email") {
            Some(user_email) => {
                user_email
            }
            None => {
                log::error!("Failed to get user_email");
                "".to_string()
            }
        };

        let user = User {
            user_id,
            user_name,
            user_access_token,
            user_token,
            user_email,
        };

        Ok(user)
    }
}

impl User {
    fn new(username: &str, access_token: &str, token: &str, email: &str) -> Self {
        User {
            user_id: 0,
            user_name: username.to_string(),
            user_access_token: access_token.to_string(),
            user_token: token.to_string(),
            user_email: email.to_string(),
        }
    }

    pub fn create_new_user(username: &str, token: &str, email: &str, access_token: &str, database_config: &DatabaseConfig) -> User {
        let mut user = User::new(username, &access_token, token, email);
        user.save(&database_config).unwrap();
        user
    }   

    pub fn id(&self) -> i32 {
        self.user_id
    }

    pub fn get_user_name(&self) -> String {
        self.user_name.clone()
    }

    pub fn get_user_access_token(&self) -> String {
        self.user_access_token.clone()
    }

    pub fn get_user_token(&self) -> String {
        self.user_token.clone()
    }

    pub fn get_user_email(&self) -> String {
        self.user_email.clone()
    }

    pub fn accept_terms(&mut self, database_config: &DatabaseConfig) -> Result<(), Box<dyn Error>> {
        let mut conn = database_config.get_connection()?;
        let query = "UPDATE USER_LIST SET user_terms_accepted = ? WHERE user_name = ?";
        let params: Params = Params::from((true, &self.user_name));
        conn.exec_first::<Row, &str, Params>(query, params)?;
        Ok(())
    }

    pub fn get_terms(&self, database_config: &DatabaseConfig) -> Result<bool, Box<dyn Error>> {
        let mut conn = database_config.get_connection()?;
        let query = "SELECT user_terms_accepted FROM USER_LIST WHERE user_id = ?";
        let params: Params = Params::from((self.user_id,));
        let terms_row = conn.exec_first::<Row, &str, Params>(query, params)?;
        let terms_row = match terms_row {
            Some(terms_row) => {
                terms_row
            },
            None => {
                return Err("Failed to get terms".into());
            }
        };

        let user_terms_accepted: bool = match terms_row.get("user_terms_accepted") {
            Some(user_terms_accepted) => {
                user_terms_accepted
            }
            None => {
                log::error!("Failed to get user_terms_accepted");
                false
            }
        };

        Ok(user_terms_accepted)
    }

    pub fn refresh_token(&mut self, token: &str, database_config: &DatabaseConfig) -> Result<(), Box<dyn Error>> {
        self.user_token = token.to_string();
        let mut conn = database_config.get_connection()?;
        let query = "UPDATE USER_LIST SET user_token = ? WHERE user_name = ?";
        let params: Params = Params::from((token, &self.user_name));
        conn.exec_first::<Row, &str, Params>(query, params)?;
        Ok(())
    }

    pub fn by_id(
        user_id: i32,
        database_config: &DatabaseConfig,
    ) -> Result<User, Box<dyn Error>> {
        // Connect to the database using the connection pool
        let mut conn = database_config
            .get_connection()
            .map_err(|err| format!("Failed to get database connection: {}", err))?;

        // Execute the SELECT query to get the user
        let query = "SELECT user_id, user_name, user_access_token, user_token, user_email FROM USER_LIST WHERE user_id = ?";
        let params = Params::from((user_id,));
        let user_row = conn
            .exec_first::<Row, &str, Params>(query, params)
            .map_err(|err| format!("Failed to execute query: {}", err))?;

        let user_row = match user_row {
            Some(user_row) => {
                user_row
            },
            None => {
                return Err("Failed to get user".into());
            }
        };

        let user = User::from_row(user_row);
        Ok(user)
    }

    pub fn by_audio_id(
        audio_id: &i32,
        database_config: &DatabaseConfig,
    ) -> std::result::Result<Option<User>, Box<dyn Error>> {
        log::info!("by_audio_id: looking up audio");
        let audio = Audio::by_id(*audio_id, database_config)?;

        log::info!("by_audio_id: looking up user");
        let mut conn = database_config.get_connection()?;
        let query = "SELECT user_id, user_name, user_access_token, user_token, user_email FROM USER_LIST WHERE user_id = ?";
        let user_id = audio.get_audio_id();
        let params = Params::from((user_id,));

        let user_row_option = match conn.exec_first::<Row, &str, Params>(query, params)? {
            Some(user_row) => {
                Some(user_row)
            },
            None => {
                None
            }
        };

        let user = match user_row_option {
            Some(user_row) => {
                User::from_row(user_row)
            },
            None => {
                return Ok(None);
            }
        };

        Ok(Some(user))
    }

    pub fn from_user_name(
        user_name: &str,
        database_config: &DatabaseConfig,
    ) -> Result<Option<User>, String> {
        // Connect to the database using the connection pool
        let mut conn = database_config
            .get_connection()
            .map_err(|err| format!("Failed to get database connection: {}", err))?;

        
        // Execute the SELECT query to get the user
        let query = "SELECT user_id, user_name, user_access_token, user_token, user_email FROM USER_LIST WHERE user_name = ?";
        let params = Params::from((user_name,));
        
        // let user = user_row_option.map(UserAccount::from_row);
        let user = conn.exec_first::<Row, &str, Params>(query, params)
            .map_err(|err| format!("(by username) Failed to execute query: {}", err))?;
        let user = match user {
            Some(user) => {
                user
            },
            None => {
                return Ok(None);
            }
        };

        let user = User::from_row(user);
        Ok(Some(user))
    }

    pub fn by_user_token(
        user_token: &str,
        database_config: &DatabaseConfig,
    ) -> Result<Option<User>, String> {
        // Connect to the database using the connection pool
        let mut conn = database_config
            .get_connection()
            .map_err(|err| format!("Failed to get database connection: {}", err))?;

        
        // Execute the SELECT query to get the user
        let query = "SELECT user_id, user_name, user_access_token, user_token, user_email FROM USER_LIST WHERE user_token = ?";
        let user_token = user_token.to_string();
        let params = Params::from((user_token,));
        
        // let user = user_row_option.map(UserAccount::from_row);
        let user = conn.exec_first::<Row, &str, Params>(query, params)
            .map_err(|err| format!("(by user_token) Failed to execute query: {}", err))?;
        let user = match user {
            Some(user) => {
                user
            },
            None => {
                return Ok(None);
            }
        };

        let user = User::from_row(user);
        Ok(Some(user))
    }

    pub fn update_all(&self, database_config: &DatabaseConfig) -> Result<(), String> {
        // Connect to the database using the connection pool
        let mut conn = database_config
            .get_connection()
            .map_err(|err| format!("Failed to get database connection: {}", err))?;

        // Execute the INSERT query to insert the user
        let query = "UPDATE USER_LIST SET user_access_token = ?, user_token = ?, user_email = ? WHERE user_name = ?";
        let params: Params = Params::from((
            self.user_access_token.clone(),
            self.user_token.clone(),
            self.user_email.clone(),
            self.user_name.clone(),
        ));
        conn.exec_first::<Row, &str, Params>(query, params)
            .map_err(|err| format!("(update_all) Failed to execute query: {}", err))?;
        Ok(())
    }

    pub fn save(&mut self, database_config: &DatabaseConfig) -> Result<(), String> {
        // Connect to the database using the connection pool
        let mut conn = database_config
            .get_connection()
            .map_err(|err| format!("Failed to get database connection: {}", err))?;

        // Execute the INSERT query to insert the user
        let query = "INSERT INTO USER_LIST (user_name, user_access_token, user_token, user_email) VALUES (?, ?, ?, ?)";
        let params: Params = Params::from((
            self.user_name.clone(),
            self.user_access_token.clone(),
            self.user_token.clone(),
            self.user_email.clone(),
        ));

        let user = conn.exec_first::<Row, &str, Params>(query, params)
            .map_err(|err| format!("(on save) Failed to execute query: {}", err))?;

        let user = match user {
            Some(user) => {
                user
            },
            None => {
                return Err("Failed to save user".to_string())
            }
        };

        let user = User::from_row(user);
        self.user_id = user.user_id;
        Ok(())

    }
}


