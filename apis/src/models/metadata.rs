use r2d2::PooledConnection;
use r2d2_mysql::MysqlConnectionManager;
use r2d2_mysql::mysql::Params;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct Metadata {
    pub metadata_id: i32,
    pub track_id: i32,
    pub user_id: i32,
    
    pub title: String,
    pub artist: String,
    
    pub uuid: String,

    //prase from string to float
    pub bpm: f32,
    pub offset: f32,
    
    pub scale: String,
    pub genre: String,
    pub artstyle: String,
    pub lyrics: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct Metatrack {
    pub title: String,
    pub artist: String,

    //prase from string to float
    pub bpm: String,
    pub offset: String,
    
    pub scale: String,
    pub genre: String,
    pub artstyle: String,
    pub lyrics: String,
}


impl From<Metadata> for Metatrack {
    fn from(metadata: Metadata) -> Self {
        Metatrack {
            title: metadata.title,
            artist: metadata.artist,

            bpm: metadata.bpm.to_string(),
            offset: metadata.offset.to_string(),

            scale: metadata.scale,
            genre: metadata.genre,
            artstyle: metadata.artstyle,
            lyrics: metadata.lyrics,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct MetatrackContructor {
    pub metatrack: Metatrack,
    pub metadata_id: i32,
    pub track_id: i32,
    pub user_id: i32,
}

use rocket::request::FromFormValue;
use rocket::http::RawStr;

use super::AppState;

impl<'v> FromFormValue<'v> for Metatrack {
    type Error = std::convert::Infallible;

    fn from_form_value(form_value: &'v RawStr) -> Result<Metatrack, Self::Error> {
        let iter = form_value.split("&");
        let mut title = String::from("");
        let mut artist = String::from("");
        let mut bpm = String::from("");
        let mut offset = String::from("");
        let mut scale = String::from("");
        let mut genre = String::from("");
        let mut artstyle = String::from("");
        let mut lyrics = String::from("");
        for item in iter {
            let mut iter2 = item.split("=");
            let key = iter2.next().unwrap();
            let value = iter2.next().unwrap();
            match key {
                "title" => title = value.to_string(),
                "artist" => artist = value.to_string(),
                "bpm" => bpm = value.to_string(),
                "offset" => offset = value.to_string(),
                "scale" => scale = value.to_string(),
                "genre" => genre = value.to_string(),
                "artstyle" => artstyle = value.to_string(),
                "lyrics" => lyrics = value.to_string(),
                _ => (),
            }
        }
        Ok(Metatrack {
            title,
            artist,
            bpm,
            offset,
            scale,
            genre,
            artstyle,
            lyrics,
        })
    }
}

impl From<MetatrackContructor> for Metadata {

    fn from(constructor: MetatrackContructor) -> Self {
        let metatrack = constructor.metatrack;
        let metadata_id = constructor.metadata_id;
        let track_id = constructor.track_id;
        let user_id = constructor.user_id;
        
        Metadata {
            metadata_id,
            track_id,
            user_id,
            
            title: metatrack.title,
            artist: metatrack.artist,
            
            uuid: "".to_string(),
            
            bpm: metatrack.bpm.parse::<f32>().unwrap(),
            offset: metatrack.offset.parse::<f32>().unwrap(),

            scale: metatrack.scale,
            genre: metatrack.genre,
            artstyle: metatrack.artstyle,
            lyrics: metatrack.lyrics,
        }
    }
}

impl Metadata {

    pub fn insert_on_db(&self, mut conn: PooledConnection<MysqlConnectionManager>, track_metadata: &Metadata) -> Result<(), Box<dyn std::error::Error>> {
        let query = "INSERT INTO TRACK_METADATA (uuid, title, artist, genre, artstyle, bpm, `offset`, scale, lyrics, user_id, track_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        let params: Params = Params::from((track_metadata.uuid.clone(), track_metadata.title.clone(), track_metadata.artist.clone(), track_metadata.genre.clone(), track_metadata.artstyle.clone(), track_metadata.bpm.clone(), track_metadata.offset.clone(), track_metadata.scale.clone(), track_metadata.lyrics.clone(), track_metadata.user_id, track_metadata.track_id));
        conn.prep_exec(query, params)?;
        Ok(())
    }
    

    pub fn update_on_db(&self, mut conn: PooledConnection<MysqlConnectionManager>, track_metadata: &Metadata) -> Result<(), Box<dyn std::error::Error>> {
        let query = "UPDATE TRACK_METADATA SET uuid=?, title=?, artist=?, genre=?, artstyle=?, bpm=?, `offset`=?, scale=?, lyrics=? WHERE user_id=? AND track_id=? AND metadata_id=?";
        let params: Params = Params::from((track_metadata.uuid.clone(), track_metadata.title.clone(), track_metadata.artist.clone(), track_metadata.genre.clone(), track_metadata.artstyle.clone(), track_metadata.bpm.clone(), track_metadata.offset.clone(), track_metadata.scale.clone(), track_metadata.lyrics.clone(), track_metadata.user_id, track_metadata.track_id, track_metadata.metadata_id));
        conn.prep_exec(query, params)?;
        Ok(())
    }

    // Function to load track metadata from the database based on user_id and track_id
    pub fn select_on_db(
        state: &AppState,
        user_id: i32,
        track_id: i32,
    ) -> Result<Metadata, Box<dyn std::error::Error>> {
        let mut conn = state.db_pool().get()?;        
        let query = "SELECT * FROM TRACK_METADATA WHERE user_id=? AND track_id=?";
        let params: Params = Params::from((user_id, track_id));
        let mut result = conn.prep_exec(query, params)?;
        let row: Option<Result<r2d2_mysql::mysql::Row, r2d2_mysql::mysql::Error>> = result.next();
        match row {
            None => {
                let metadata = Metadata {
                    metadata_id: 0,
                    track_id,
                    user_id,
                    
                    title: "".to_string(),
                    artist: "".to_string(),
                    
                    uuid: "".to_string(),
                    
                    bpm: 0.0,
                    offset: 0.0,
                    
                    scale: "".to_string(),
                    genre: "".to_string(),
                    artstyle: "".to_string(),
                    lyrics: "".to_string(),
                };
                
                println!("No metadata found for user_id {} and track_id {}", user_id, track_id);
                let mut conn = state.db_pool().get()?;  
                metadata.insert_on_db(conn, &metadata)?;
                Ok(metadata)

            }

            Some(row) => {
                let row = row?;
              
                Ok(Metadata {
                    metadata_id: row.get("metadata_id").unwrap(),
                    track_id,
                    user_id,
                    
                    title: row.get("title").unwrap(),
                    artist: row.get("artist").unwrap(),
                    
                    uuid: row.get("uuid").unwrap(),
                    
                    bpm: row.get("bpm").unwrap(),
                    offset: row.get("offset").unwrap(),
                    
                    scale: row.get("scale").unwrap(),
                    genre: row.get("genre").unwrap(),
                    artstyle: row.get("artstyle").unwrap(),
                    lyrics: row.get("lyrics").unwrap(),
                })
            }
        }
    }
}