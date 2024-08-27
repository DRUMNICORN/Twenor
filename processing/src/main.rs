use std::time::Duration;
use models::{AudioState, db};
use crate::{state_manager::StateManager, utils::init_logging};

mod state_manager;
mod utils;
mod handlers;

#[tokio::main]
async fn main() {
    match init_logging() {
        Ok(_) => log::info!("Logging initialized"),
        Err(e) => {
            panic!("Error initializing logging: {}", e);
        }
    };
    dotenv::dotenv().ok();
    let database_config = match db::DatabaseConfig::new() {
        Ok(config) => config,
        Err(err) => {
            log::error!("Error connecting to database: {}", err);
            panic!("Error connecting to database: {}", err);
        }
    };
    
    let audio_states = vec![
        AudioState::Uploading,

        AudioState::Describing,
        AudioState::Chunking,
        // AudioState::Converting,

        AudioState::Featuring,
        AudioState::Correlating,
        AudioState::Splitting,
        AudioState::Writing,

        AudioState::Prompting,
        AudioState::Rendering,
    ];


    let state_manager = StateManager::new(database_config.clone());

    log::info!("Starting loop to search for audios in states: {:?}", audio_states);
    loop {
        for state in &audio_states {
            log::debug!("Searching for audios in state: {}", state.as_str());
            // if let Some(audio_id) = match AudioState::by_state(state.clone(), &database_config){
            //     Ok(audio_id) => audio_id,
            //     Err(e) => {
            //         log::error!("Failed to get audio in state {}: {}", state.as_str(), e);
            //         continue
            //     }               
            // } {
            //     state_manager.handle_state_transition(state.clone(), audio_id).await;
            // }
            match AudioState::all_by_state(state.clone(), &database_config){
                Ok(audio_id) => for audio_id in audio_id {
                    log::debug!("Found audio in state {}: {}", state.as_str(), audio_id);
                    state_manager.handle_state_transition(state.clone(), audio_id).await;
                },
                Err(e) => {
                    log::error!("Failed to get audio in state {}: {}", state.as_str(), e);
                    continue
                }               
            }
        }

        log::debug!("Sleeping for 100ms");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
