use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct Notification {
    pub title: String,
    pub description: String
}

#[derive(Clone, Data, Lens)]
pub struct CurrentlyPlaying {
    pub title: String,
    pub artist: String,
    pub artwork: String,
    pub pause_enabled: bool,
    pub move_forward: bool,
    pub move_backward: bool,
    pub seek: bool,
    pub progress: f64
}