use bevy::{
    math::{uvec2, vec3},
    prelude::{IVec2, Res, UVec2, Vec2, Vec3},
    window::Window,
};
use serde::{Deserialize, Serialize};

/// Tile size options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    /// Fixed tile size
    Fixed(f32),
    /// Window adaptative tile size
    Adaptive { min: f32, max: f32 },
}

/// Board position customization options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardPosition {
    /// Centered board
    Centered { offset: Vec3 },
    /// Custom position
    Custom(Vec3),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Difficulty {
    /// Tile map size
    pub dim: UVec2,
    /// bomb count
    pub n_mines: u32,
}

impl Difficulty {
    pub const EASY: Self = Self {
        dim: uvec2(9, 9),
        n_mines: 10,
    };

    pub const MEDIUM: Self = Self {
        dim: uvec2(16, 16),
        n_mines: 40,
    };

    pub const EXPERT: Self = Self {
        dim: uvec2(30, 16),
        n_mines: 99,
    };
}

/// Board generation options. Must be used as a resource
// We use serde to allow saving option presets and loading them at runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardOptions {
    pub difficulty: Difficulty,
    /// Board world position
    pub position: BoardPosition,
    /// Tile world size
    pub tile_size: TileSize,
    /// Padding between tiles
    pub tile_padding: f32,
    /// Does the board generate a safe place to start
    pub safe_start: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct DisplayParams {
    pub board_size: Vec2,
    pub tile_size: f32,
    pub position: Vec3,
}

impl BoardOptions {
    pub fn display_params(&self, window_dim: Vec2) -> DisplayParams {
        let tile_size = match self.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { min, max } => {
                let [max_width, max_height] =
                    (window_dim / self.difficulty.dim.as_vec2()).to_array();
                max_width.min(max_height).clamp(min, max)
            }
        };

        let board_size = self.difficulty.dim.as_vec2() * tile_size;
        let position = match self.position {
            BoardPosition::Centered { offset } => -board_size.extend(0.0) / 2.0 + offset,
            BoardPosition::Custom(p) => p,
        };

        DisplayParams {
            board_size,
            tile_size,
            position,
        }
    }
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 50.0,
        }
    }
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Default::default(),
        }
    }
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            difficulty: Difficulty::MEDIUM,
            position: Default::default(),
            tile_size: Default::default(),
            tile_padding: 0.,
            safe_start: true,
        }
    }
}
