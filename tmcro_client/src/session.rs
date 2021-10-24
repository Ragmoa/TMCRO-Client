mod instruction;

use crate::session::instruction::Instruction as Instruction;

// Represents an online session
struct Session {
    roomName: String,
    playerName: String,
    gameMode: GameMode
}

enum GameMode{
    Coop
    {
        unSavedInstructions: Vec<Instruction>
    }
}
