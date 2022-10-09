use bevy::{prelude::Plugin, window::Windows};

use bevy::{
    prelude::{info, BuildChildren, Color, Commands, GlobalTransform, Name, Res, Transform, Vec2},
    sprite::{Sprite, SpriteBundle},
};
use tap::Pipe;

use crate::{
    components::BoardCoordinate,
    resources::{
        board::TileMap,
        board_options::{BoardOptions, DisplayParams},
    },
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(Self::create_board);
    }
}

impl BoardPlugin {
    pub fn create_board(
        mut cmds: Commands,
        board_options: Option<Res<BoardOptions>>,
        windows: Res<Windows>,
    ) {
        let options = board_options.map(|res| res.clone()).unwrap_or_default();

        let mut tile_map = TileMap::from_options(&options);
        #[cfg(feature = "debug")]
        info!("{:#}", tile_map);

        let window_dim = windows
            .get_primary()
            .unwrap()
            .pipe(|window| [window.width(), window.height()].into());

        let DisplayParams {
            board_size,
            tile_size,
            position,
        } = options.display_params(window_dim);

        cmds.spawn()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: board_size.into(),
                            ..Default::default()
                        },
                        transform: Transform::from_translation(board_size.extend(0.0) / 2.0),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));
            })
            .with_children(|parent| {
                tile_map.all_tiles().for_each(|tile| {
                    parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: Color::GRAY,
                                custom_size: Vec2::splat(tile_size - options.tile_padding).into(),
                                ..Default::default()
                            },
                            transform: Transform::from_translation({
                                let coord = tile.coord().as_vec2() * tile_size + (tile_size / 2.0);
                                coord.extend(1.0)
                            }),
                            ..Default::default()
                        })
                        .insert(Name::new(format!("Tile {:?}", tile.coord().to_array())))
                        .insert(BoardCoordinate {
                            inner: tile.coord(),
                        });
                });
            });
    }
}
