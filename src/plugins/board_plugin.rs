use bevy::{
    prelude::{AssetServer, ChildBuilder, Handle, Image, Plugin, SpatialBundle, Vec3, Visibility},
    text::{Font, Text, Text2dBundle, TextAlignment, TextStyle},
    window::Windows,
};

use bevy::{
    prelude::{info, BuildChildren, Color, Commands, GlobalTransform, Name, Res, Transform, Vec2},
    sprite::{Sprite, SpriteBundle},
};
use tap::{Pipe, Tap};

use crate::{
    components::{BoardCoordinate, Mine, MineNeighbor},
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
        asset_server: Res<AssetServer>,
    ) {
        let font: Handle<Font> = asset_server.load("fonts/robotoslab.ttf");
        let mine_image: Handle<Image> = asset_server.load("sprites/bomb.png");
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
            .insert_bundle(SpatialBundle {
                visibility: Visibility::visible(),
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            .with_children(Self::spawn_background(board_size))
            .with_children(Self::spawn_tiles(
                &mut tile_map,
                tile_size,
                options.tile_padding,
                mine_image,
                font,
            ));
    }

    fn spawn_background(size: Vec2) -> impl FnOnce(&mut ChildBuilder) {
        move |parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: size.into(),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(size.extend(0.0) / 2.0),
                    ..Default::default()
                })
                .insert(Name::new("Background"));
        }
    }

    fn spawn_tiles(
        tile_map: &mut TileMap,
        tile_size: f32,
        tile_padding: f32,
        mine_image: Handle<Image>,
        font: Handle<Font>,
    ) -> impl FnOnce(&mut ChildBuilder) + '_ {
        let sprite_size = Vec2::splat(tile_size - tile_padding);

        move |parent| {
            tile_map.all_tiles().for_each(|tile| {
                let mut tile_entity = parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GRAY,
                        custom_size: sprite_size.into(),
                        ..Default::default()
                    },
                    transform: Transform::from_translation({
                        let coord = tile.coord().as_vec2() * tile_size + (tile_size / 2.0);
                        coord.extend(1.0)
                    }),
                    ..Default::default()
                });

                tile_entity
                    .insert(Name::new(format!("Tile {:?}", tile.coord().to_array())))
                    .insert(BoardCoordinate {
                        inner: tile.coord(),
                    });

                match tile.state() {
                    crate::resources::board::TileState::Mine => {
                        tile_entity.insert(Mine).with_children(|parent| {
                            parent.spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: sprite_size.into(),
                                    ..Default::default()
                                },
                                transform: Transform::from_translation(Vec3::Z),
                                texture: mine_image.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    crate::resources::board::TileState::Clear(n) if n > 0 => {
                        tile_entity.insert(MineNeighbor(n)).with_children(|parent| {
                            parent.spawn_bundle(Text2dBundle {
                                text: Text::from_section(
                                    n.to_string(),
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: sprite_size.x,
                                        color: match n {
                                            // 0 => Color::BLACK,
                                            1 => Color::BLUE,
                                            2 => Color::GREEN,
                                            3 => Color::ORANGE,
                                            _ => Color::RED,
                                        },
                                    },
                                )
                                .with_alignment(TextAlignment::CENTER),
                                transform: Transform::from_translation(Vec3::Z),
                                ..Default::default()
                            });
                        });
                    }
                    _ => {}
                }
            });
        }
    }
}
