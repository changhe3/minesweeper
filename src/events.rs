use bevy::prelude::IVec2;

#[derive(Debug, Copy, Clone)]
pub struct BoardClearEvent;

#[derive(Debug, Copy, Clone)]
pub struct MineTriggerEvent;

#[derive(Debug, Copy, Clone)]
pub struct TileMarkEvent {
    pub coord: IVec2,
}
