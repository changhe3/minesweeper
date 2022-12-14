use std::{
    cell::Cell,
    fmt::{Debug, Display},
    ops::DerefMut,
};

use bevy::prelude::{IVec2, UVec2};
use colored::Colorize;
use itertools::Itertools;
use nanorand::{tls_rng, Rng};

use super::board_options::{BoardOptions, Difficulty};

#[must_use]
fn bound_check(coord: IVec2, dim: IVec2) -> bool {
    coord.cmpge(IVec2::ZERO).all() && coord.cmplt(dim).all()
}

fn bound_check_assert(coord: IVec2, dim: IVec2) {
    assert!(
        bound_check(coord, dim),
        "Coordinate {:?} must be bound between [0, 0] and {:?}",
        coord.to_array(),
        dim.to_array()
    );
}

#[derive(Debug, Clone)]
pub struct TileMap {
    n_mines: u32,

    // (width, height)
    dim: IVec2,

    // number of adjacent mines, negative if the tile itself is a mine
    tiles: Box<[i8]>,
}

impl TileMap {
    pub fn empty(width: u32, height: u32) -> Self {
        Self {
            n_mines: 0,
            dim: IVec2::new(width.try_into().unwrap(), height.try_into().unwrap()),
            tiles: vec![0; (width * height) as usize].into_boxed_slice(),
        }
    }

    pub fn random(width: u32, height: u32, n_mines: u32) -> Self {
        let mut board = Self::empty(width, height);
        let mut rng = tls_rng();
        board.tiles[..n_mines as usize].fill(-1);
        rng.shuffle(&mut board.tiles);

        board
            .all_tiles()
            .filter(|tile| !tile.is_mine())
            .for_each(|tile| {
                let adj_mines = tile.neighbors().filter(|tile| tile.is_mine()).count();
                tile.tile_state().set(adj_mines as i8);
            });

        board
    }

    pub fn from_options(options: &BoardOptions) -> Self {
        let Difficulty {
            dim: UVec2 { x, y },
            n_mines,
        } = options.difficulty;

        Self::random(x, y, n_mines)
    }

    pub fn width(&self) -> u32 {
        self.dim.x as u32
    }

    pub fn height(&self) -> u32 {
        self.dim.y as u32
    }

    pub fn dim(&self) -> IVec2 {
        self.dim
    }

    pub fn n_mines(&self) -> u32 {
        self.n_mines
    }

    fn tile_state(&self, coord: IVec2) -> i8 {
        let width = self.dim.x;

        bound_check_assert(coord, self.dim);

        let idx = coord.y * width + coord.x;
        self.tiles[idx as usize]
    }

    pub fn get_tile<T: Into<IVec2>>(&mut self, coord: T) -> Option<TileView> {
        fn get_tile(inner: &mut TileMap, coord: IVec2) -> Option<TileView> {
            bound_check(coord, inner.dim).then(|| TileView {
                coord,
                n_mines: inner.n_mines,
                dim: inner.dim,
                tiles: Cell::from_mut(inner.tiles.deref_mut()).as_slice_of_cells(),
            })
        }

        get_tile(self, coord.into())
    }

    pub fn tile<T: Into<IVec2>>(&mut self, coord: T) -> TileView {
        self.get_tile(coord).unwrap()
    }

    pub fn get_tiles<T: Into<IVec2>>(
        &mut self,
        coords: impl Iterator<Item = T>,
    ) -> impl Iterator<Item = Option<TileView<'_>>> {
        let tiles = Cell::from_mut(self.tiles.deref_mut()).as_slice_of_cells();
        coords.map_into().map(|coord| {
            bound_check(coord, self.dim).then_some(TileView {
                coord,
                n_mines: self.n_mines,
                dim: self.dim,
                tiles,
            })
        })
    }

    pub fn coords(&self) -> impl Iterator<Item = IVec2> {
        let [width, height] = self.dim.to_array();
        (0..height)
            .cartesian_product(0..width)
            .map(|(y, x)| (x, y).into())
    }

    pub fn all_tiles(&mut self) -> impl Iterator<Item = TileView<'_>> {
        let coords = self.coords();
        self.get_tiles(coords).map(Option::unwrap)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TileView<'a> {
    coord: IVec2,
    n_mines: u32,

    // (width, height)
    dim: IVec2,

    // number of adjacent mines, negative if the tile itself is a mine
    tiles: &'a [Cell<i8>],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileState {
    Mine,
    Clear(u8),
}

impl<'a> TileView<'a> {
    fn tile_state(&self) -> &Cell<i8> {
        let width = self.dim.x;

        let idx = self.coord.y * width + self.coord.x;
        &self.tiles[idx as usize]
    }

    pub fn state(&self) -> TileState {
        match self.tile_state().get() {
            n if n < 0 => TileState::Mine,
            n => TileState::Clear(n as u8),
        }
    }

    pub fn set_state(&self, state: TileState) {
        let tile = self.tile_state();

        match state {
            TileState::Mine => tile.set(-1),
            TileState::Clear(n) => tile.set(n as i8),
        }
    }

    pub fn is_mine(&self) -> bool {
        self.state() == TileState::Mine
    }

    pub fn coord(&self) -> IVec2 {
        self.coord
    }

    pub fn dim(&self) -> IVec2 {
        self.dim
    }

    pub fn with_coordinate<T: Into<IVec2>>(self, coord: T) -> Self {
        fn with_coordinate(mut this: TileView, coord: IVec2) -> TileView {
            bound_check_assert(coord, this.dim);

            this.coord = coord;
            this
        }

        with_coordinate(self, coord.into())
    }

    pub fn try_with_coordinate<T: Into<IVec2>>(self, coord: T) -> Option<Self> {
        fn try_with_coordinate(mut this: TileView, coord: IVec2) -> Option<TileView> {
            bound_check(coord, this.dim).then(|| {
                this.coord = coord;
                this
            })
        }

        try_with_coordinate(self, coord.into())
    }

    pub fn step<T: Into<IVec2>>(self, coord: T) -> Option<Self> {
        self.try_with_coordinate(coord.into() + self.coord)
    }

    pub fn neighbors(self) -> impl Iterator<Item = TileView<'a>> {
        /// Delta coordinates for all 8 square neighbors
        const NEIGHBORS: [[i32; 2]; 8] = [
            [-1, -1],
            [0, -1],
            [1, -1],
            [-1, 0],
            [1, 0],
            [-1, 1],
            [0, 1],
            [1, 1],
        ];

        NEIGHBORS.into_iter().filter_map(move |delta| {
            let coord = self.coord + IVec2::from(delta);
            bound_check(coord, self.dim).then(|| self.with_coordinate(coord))
        })
    }
}

impl Display for TileMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct Map<'a> {
            inner: &'a TileMap,
        }

        impl Debug for Map<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut builder = f.debug_list();

                self.inner
                    .tiles
                    .chunks(self.inner.width() as usize)
                    .for_each(|row| {
                        let fmt = row.iter().format_with(" ", |&tile, f| {
                            f(&format_args!(
                                "{}",
                                match tile {
                                    0 => " ".normal(),
                                    1 => "1".cyan(),
                                    2 => "2".green(),
                                    3 => "3".yellow(),
                                    other if other >= 0 => other.to_string().red(),
                                    _ => "*".bright_red(),
                                }
                            ))
                        });
                        builder.entry(&format_args!("| {} |", fmt));
                    });

                builder.finish()
            }
        }

        let mut builder = f.debug_struct("TileMap");
        builder.field("width", &self.dim.x);
        builder.field("height", &self.dim.y);
        builder.field("map", &Map { inner: self });
        builder.finish()
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::TileMap;

    #[test]
    fn test_neighbors() {
        let mut tiles = TileMap::empty(8, 8);

        let tile = tiles.tile([1, 1]);
        let actual = tile
            .neighbors()
            .map(|tile| tile.coord.to_array())
            .sorted()
            .collect_vec();

        let expected = [
            [0, 0],
            [1, 0],
            [2, 0],
            [0, 1],
            [2, 1],
            [0, 2],
            [1, 2],
            [2, 2],
        ]
        .into_iter()
        .sorted()
        .collect_vec();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_random() {
        let board = TileMap::random(30, 16, 99);
        println!("{:#}", board);
    }
}
