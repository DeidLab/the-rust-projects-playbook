use bevy::{prelude::*, utils::HashMap};
use board::*;
use rand::Rng;
use system::*;

mod board;
mod matches;
mod system;

pub mod prelude {
    pub use crate::board::*;
    pub use crate::matches::*;
    pub use crate::system::*;
    pub use crate::Match3Plugin;
    pub use crate::Match3Config;
}

pub struct Match3Plugin;

impl Plugin for Match3Plugin {
    fn build(&self, app: &mut App) {
        let Match3Config {
            board_dimensions,
            gem_types,
        } = app
            .world
            .get_resource::<Match3Config>()
            .copied()
            .unwrap_or_default();

        if gem_types < 3 {
            panic!("Cannot generate board with fewer than 3 different gem types");
        }

        let mut gems = HashMap::default();
        (0..board_dimensions.x).for_each(|x| {
            (0..board_dimensions.y).for_each(|y| {
                gems.insert([x, y].into(), rand::thread_rng().gen_range(0..gem_types));
            })
        });

        let mut board = Board {
            dimensions: board_dimensions,
            gems,
            types: (0..gem_types).collect(),
        };

        board.clear_matches();

        app.insert_resource(board)
            .insert_resource(BoardCommands::default())
            .insert_resource(BoardEvents::default())
            .add_systems(Update, read_commands);
    }
}

#[derive(Clone, Copy, Resource)]
pub struct Match3Config {
    /// The number of different gem types the board can spawn
    pub gem_types: u32,
    /// The rectangular dimensions of the board
    pub board_dimensions: UVec2,
}

impl Default for Match3Config {
    fn default() -> Self {
        Self {
            gem_types: 5,
            board_dimensions: [10, 10].into(),
        }
    }
}