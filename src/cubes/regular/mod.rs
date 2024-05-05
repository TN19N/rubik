mod moves;
mod utils;

const CUBE_SIDES: usize = 6;

#[derive(Debug, Clone, Copy)]
enum CubePieceColor {
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    White,
}

type CubeSide = Vec<Vec<CubePieceColor>>;

#[derive(Debug)]
pub struct Cube {
    data: [CubeSide; CUBE_SIDES],
}

impl Cube {
    pub fn new(size: usize) -> Self {
        let create_cube_side = |color: CubePieceColor| -> CubeSide {
            (0..size)
                .map(|_| (0..size).map(|_| color).collect())
                .collect()
        };

        Self {
            data: [
                create_cube_side(CubePieceColor::Red),
                create_cube_side(CubePieceColor::Blue),
                create_cube_side(CubePieceColor::Green),
                create_cube_side(CubePieceColor::Yellow),
                create_cube_side(CubePieceColor::Orange),
                create_cube_side(CubePieceColor::White),
            ],
        }
    }
}
