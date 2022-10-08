use std::num::NonZeroU8;

use bevy::prelude::{Component, UVec2};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BoardCoordinate {
    pub inner: UVec2,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct Mine;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MineNeighbor(NonZeroU8);
