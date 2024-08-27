use super::Metadata;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrackState {
    None,
    Queued,

    Loading,
    
    Features,
    
    Corrolation,
    
    Arranging,

    Writing,
    Prompting,

    Rendering,
    
    Done,
}

impl TrackState {
    pub fn as_str(&self) -> &str {
        match self {
            TrackState::None => "none",
            TrackState::Queued => "queued",
            TrackState::Loading => "loading",
            TrackState::Features => "features",
            TrackState::Corrolation => "corrolation",
            TrackState::Arranging => "arranging",
            TrackState::Writing => "writing",
            TrackState::Rendering => "rendering",
            TrackState::Prompting => "prompting",
            TrackState::Done => "done",
        }
    }

    pub fn from_changed_metadata(old: Metadata, new: Metadata, current_state: TrackState) -> TrackState {
        let title_changed = old.title != new.title;
        let artist_changed = old.artist != new.artist;
        let genre_changed = old.genre != new.genre;
        let artstyle_changed = old.artstyle != new.artstyle;
        let bpm_changed = old.bpm != new.bpm;
        let offset_changed = old.offset != new.offset;
        let scale_changed = old.scale != new.scale;
        let lyrics_changed = old.lyrics != new.lyrics;

        println!("current_state: {}", current_state.as_str());

        if title_changed || artist_changed || genre_changed || artstyle_changed || scale_changed || lyrics_changed {
            println!("Metadata changed");
        } else if bpm_changed || offset_changed {
            println!("BPM or offset changed");
        } else {
            println!("No metadata changed");
            // TrackState::Done
        }
        // TODO: Add more metadata fields
        TrackState::Queued
    }
}


