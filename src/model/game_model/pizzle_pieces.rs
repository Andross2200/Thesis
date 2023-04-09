use bevy::prelude::*;

#[derive(Component)]
pub struct PuzzlePiece;

pub trait Action {
    fn get_action(&self) -> String;
}

#[derive(Component)]
pub struct MovementPuzzlePiece {
    pub direction: String,
    pub pawn_color: String,
}

impl Action for MovementPuzzlePiece {
    fn get_action(&self) -> String {
        format!("m{},{})", self.direction, self.pawn_color)
    }
}

impl MovementPuzzlePiece {}

#[derive(Component)]
pub struct CollectPerlPuzzlePiece {
    pub pawn_color: String,
}

impl Action for CollectPerlPuzzlePiece {
    fn get_action(&self) -> String {
        format!("c{}p", self.pawn_color)
    }
}

impl CollectPerlPuzzlePiece {}
