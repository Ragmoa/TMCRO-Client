
pub enum ItemOrigin{
    //Item found in player's game
    LocalItemOrigin,
    // Item given by another player
    DistItemOrigin,
    // Item given by debug
    DebugItemOrigin,
    // Other
    OtherItemOrigin,
}