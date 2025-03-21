use cgmath::Vector2;
use serde::{Deserialize, Serialize};

use crate::juno::grid::{Grid, GridItem};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StaticTileInfo {
    x: i32,
    y: i32,
    down: String,
    left: String,
    up: String,
    right: String,
}

impl StaticTileInfo {
    pub fn new(x: i32, y: i32, down: String, left: String, up: String, right: String) -> Self {
        Self {
            x,
            y,
            down,
            left,
            up,
            right,
        }
    }

    pub fn pos(&self) -> Vector2<i32> {
        Vector2::new(self.x, self.y)
    }

    pub fn down(&self) -> &String {
        &self.down
    }

    pub fn left(&self) -> &String {
        &self.left
    }

    pub fn right(&self) -> &String {
        &self.right
    }

    pub fn up(&self) -> &String {
        &self.up
    }
}

// Tile for multithreading generation
#[derive(Clone, Debug)]
pub struct GenTile {
    // Contents
    static_tile: Option<StaticTileInfo>,
    entropy: Entropy,
}

impl GenTile {
    pub fn new(static_tile: StaticTileInfo) -> Self {
        Self {
            static_tile: Some(static_tile),
            entropy: Entropy::default(),
        }
    }

    pub fn empty() -> Self {
        Self {
            static_tile: None,
            entropy: Entropy::Uncalulated,
        }
    }

    pub fn static_tile(&self) -> &Option<StaticTileInfo> {
        &self.static_tile
    }

    pub fn set_static_tile(&mut self, tile: StaticTileInfo) {
        self.static_tile = Some(tile);
    }

    pub fn entropy(&self) -> Entropy {
        self.entropy
    }

    pub fn set_entropy(&mut self, entropy: u32) {
        self.entropy = Entropy::Calculated(entropy);
    }

    pub fn remove_entropy(&mut self) {
        self.entropy = Entropy::Set;
    }

    pub fn min_entropy<'a>(
        lhs: &'a GridItem<GenTile>,
        rhs: &'a GridItem<GenTile>,
    ) -> &'a GridItem<GenTile> {
        let lhs_entropy = match lhs.contents().entropy {
            Entropy::Calculated(e) => e,
            _ => u32::MAX,
        };
        let rhs_entropy = match rhs.contents().entropy {
            Entropy::Calculated(e) => e,
            _ => u32::MAX,
        };
        match lhs_entropy < rhs_entropy {
            true => lhs,
            false => rhs,
        }
    }

    pub fn tile_set(&self) -> bool {
        self.static_tile().is_some()
    }
}

#[derive(Clone, Debug)]
pub struct Subsector {
    // Execution stage
    stage: GenerationStage,
    // Tile within stage
    tiles: Grid<GenTile>,
}

impl Subsector {
    pub fn new(stage: GenerationStage, tiles: Grid<GenTile>) -> Self {
        Self { stage, tiles }
    }

    pub fn grid(&self) -> &Grid<GenTile> {
        &self.tiles
    }

    pub fn generation_stage(&self) -> &GenerationStage {
        &self.stage
    }

    pub fn grid_mut(&mut self) -> &mut Grid<GenTile> {
        &mut self.tiles
    }
}

// The largest sectors can be done individually, all at once. Then we have to get progressively
// smaller in our chunk sizes to allow blending finer points. See https://imgur.com/a/Vts2yFe for a
// diagram —  red sectors are done first, blue sectors second, and green sectors third.
#[derive(Clone, Debug, PartialEq)]
pub enum GenerationStage {
    Primary,
    SecondaryHorizontal,
    SecondaryVertical,
    Tertiary,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Entropy {
    #[default]
    Uncalulated,
    Calculated(u32),
    Set,
}
