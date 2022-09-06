use bevy::reflect::TypeUuid;


#[derive(TypeUuid, Debug)]
#[uuid = "509d81bd-432c-4384-a6b8-65d22f638004"]
pub enum GameState {
    Normal,
    Paused,
}


impl Default for GameState {
    fn default() -> Self {
        GameState::Normal
    }
}


impl GameState {
    pub fn pause(&mut self) ->&mut Self {
        *self = Self::Paused;
        self
    }

    pub fn resume(&mut self) ->&mut Self {
        *self = Self::Normal;
        self
    }

    /// Returns `true` if the game state is [`Paused`].
    ///
    /// [`Paused`]: GameState::Paused
    pub fn is_paused(&self) -> bool {
        matches!(self, Self::Paused)
    }

    /// Returns `true` if the game state is [`Normal`].
    ///
    /// [`Normal`]: GameState::Normal
    pub fn is_normal(&self) -> bool {
        matches!(self, Self::Normal)
    }
}
