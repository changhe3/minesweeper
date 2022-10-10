use bevy::prelude::{Component, IVec2, Plugin};

#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;
use bevy_inspector_egui::RegisterInspectable;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BoardCoordinate {
    pub inner: IVec2,
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct Mine;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MineNeighbor(pub u8);

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Uncover;

pub struct InspectablePlugin;

impl Plugin for InspectablePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<BoardCoordinate>()
                .register_inspectable::<Mine>()
                .register_inspectable::<MineNeighbor>()
                .register_inspectable::<Uncover>();
        }
    }
}
