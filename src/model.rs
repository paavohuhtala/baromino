// This module implements types modelling the tiles and game state of Kingdomino

use std::collections::HashMap;

use tinyvec::ArrayVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Forest,
    Wheat,
    Water,
    Grassland,
    Swamp,
    Mountain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnyTileType {
    Castle,
    Domino(TileType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DominoSide {
    pub tile_type: TileType,
    pub crown_count: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Domino(pub DominoSide, pub DominoSide);

impl Domino {
    pub const fn flip(&self) -> Self {
        Self(self.1, self.0)
    }
}

// TODO: Since there is a limited number of unique tiles, we could encode all dominoes as a single byte
// (as an index to a static array of all dominoes) instead of using a struct

const fn domino(tile1: TileType, crown1: u8, tile2: TileType, crown2: u8) -> Domino {
    Domino(
        DominoSide {
            tile_type: tile1,
            crown_count: crown1,
        },
        DominoSide {
            tile_type: tile2,
            crown_count: crown2,
        },
    )
}

pub const ALL_TILES: [Domino; 48] = [
    domino(TileType::Wheat, 0, TileType::Wheat, 0),
    domino(TileType::Wheat, 0, TileType::Wheat, 0),
    domino(TileType::Forest, 0, TileType::Forest, 0),
    domino(TileType::Forest, 0, TileType::Forest, 0),
    domino(TileType::Forest, 0, TileType::Forest, 0),
    domino(TileType::Forest, 0, TileType::Forest, 0),
    domino(TileType::Water, 0, TileType::Water, 0),
    domino(TileType::Water, 0, TileType::Water, 0),
    domino(TileType::Water, 0, TileType::Water, 0),
    domino(TileType::Grassland, 0, TileType::Grassland, 0),
    domino(TileType::Grassland, 0, TileType::Grassland, 0),
    domino(TileType::Swamp, 0, TileType::Swamp, 0),
    domino(TileType::Wheat, 0, TileType::Forest, 0),
    domino(TileType::Wheat, 0, TileType::Water, 0),
    domino(TileType::Wheat, 0, TileType::Grassland, 0),
    domino(TileType::Wheat, 0, TileType::Swamp, 0),
    domino(TileType::Forest, 0, TileType::Water, 0),
    domino(TileType::Forest, 0, TileType::Grassland, 0),
    domino(TileType::Wheat, 1, TileType::Forest, 0),
    domino(TileType::Wheat, 1, TileType::Water, 0),
    domino(TileType::Wheat, 1, TileType::Grassland, 0),
    domino(TileType::Wheat, 1, TileType::Swamp, 0),
    domino(TileType::Wheat, 1, TileType::Mountain, 0),
    domino(TileType::Forest, 1, TileType::Wheat, 0),
    domino(TileType::Forest, 1, TileType::Wheat, 0),
    domino(TileType::Forest, 1, TileType::Wheat, 0),
    domino(TileType::Forest, 1, TileType::Wheat, 0),
    domino(TileType::Forest, 1, TileType::Water, 0),
    domino(TileType::Forest, 1, TileType::Grassland, 0),
    domino(TileType::Water, 1, TileType::Wheat, 0),
    domino(TileType::Water, 1, TileType::Wheat, 0),
    domino(TileType::Water, 1, TileType::Forest, 0),
    domino(TileType::Water, 1, TileType::Forest, 0),
    domino(TileType::Water, 1, TileType::Forest, 0),
    domino(TileType::Water, 1, TileType::Forest, 0),
    domino(TileType::Wheat, 0, TileType::Grassland, 1),
    domino(TileType::Water, 0, TileType::Grassland, 1),
    domino(TileType::Wheat, 0, TileType::Swamp, 1),
    domino(TileType::Grassland, 0, TileType::Swamp, 1),
    domino(TileType::Mountain, 1, TileType::Wheat, 1),
    domino(TileType::Wheat, 0, TileType::Grassland, 2),
    domino(TileType::Water, 0, TileType::Grassland, 2),
    domino(TileType::Wheat, 0, TileType::Swamp, 2),
    domino(TileType::Grassland, 0, TileType::Swamp, 2),
    domino(TileType::Mountain, 2, TileType::Wheat, 0),
    domino(TileType::Swamp, 0, TileType::Mountain, 2),
    domino(TileType::Swamp, 0, TileType::Mountain, 2),
    domino(TileType::Wheat, 0, TileType::Mountain, 3),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Castle,
    Domino(Domino),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileOrientation {
    /// The tile is oriented with the first side on the left and the second side on the right
    LeftRight,
    /// The tile is oriented with the first side on the top and the second side on the bottom
    /// This is equivalent to rotating the tile 90 degrees clockwise
    TopBottom,
    /// The tile is oriented with the first side on the right and the second side on the left
    /// This is equivalent to rotating the tile 180 degrees
    RightLeft,
    /// The tile is oriented with the first side on the bottom and the second side on the top
    /// This is equivalent to rotating the tile 270 degrees clockwise, or 90 degrees counter-clockwise
    BottomTop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position(i8, i8);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TilePlacement {
    pub tile: Tile,
    /// The top-left corner of the tile
    pub position: Position,
    pub orientation: TileOrientation,
}

pub enum TilePlacementError {
    OverlapsExistingTile,
    NoMatchingAdjacentTile,
    OutOfBounds,
}

// TODO: Support the 7x7 variant as well
const KINGDOM_MAX_SIZE: u8 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PlacementIndex(u8);

#[derive(Debug)]
pub struct Kingdom {
    placements: Vec<TilePlacement>,
    grid: HashMap<(i8, i8), PlacementIndex>,
}

impl Kingdom {
    pub fn new() -> Self {
        let initial_placement = TilePlacement {
            tile: Tile::Castle,
            position: Position(0, 0),
            orientation: TileOrientation::LeftRight,
        };

        Self {
            placements: vec![initial_placement],
            grid: HashMap::from([((0, 0), PlacementIndex(0))]),
        }
    }

    fn get_positions_filled_by_placement(
        &self,
        placement: &TilePlacement,
    ) -> ArrayVec<[Position; 2]> {
        let mut positions = ArrayVec::new();

        if let Tile::Castle = placement.tile {
            positions.push(placement.position);
            return positions;
        }

        let Position(x, y) = placement.position;

        positions.push(Position(x, y));

        match placement.orientation {
            TileOrientation::LeftRight => {
                positions.push(Position(x + 1, y));
            }
            TileOrientation::TopBottom => {
                positions.push(Position(x, y - 1));
            }
            TileOrientation::RightLeft => {
                positions.push(Position(x - 1, y));
            }
            TileOrientation::BottomTop => {
                positions.push(Position(x, y + 1));
            }
        }

        positions
    }

    fn get_adjacent_positions(&self, position: Position) -> [Position; 4] {
        let Position(x, y) = position;

        [
            Position(x + 1, y),
            Position(x, y - 1),
            Position(x - 1, y),
            Position(x, y + 1),
        ]
    }
}
