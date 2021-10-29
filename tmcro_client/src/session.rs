mod instruction;

pub use crate::session::instruction::Instruction as Instruction;

// Represents an online session
struct Session {
    room_name: String,
    player_name: String,
    game_mode: GameMode
}


enum GameMode{
    Coop
    {
        unsaved_instructions: Vec<Instruction>
    }
}
