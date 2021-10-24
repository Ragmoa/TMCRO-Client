//Orders that can be sent to the bridge
pub enum Instruction{
    WriteInstruction{
        address: u32,
        value: u8,
    },
    WatchByteInstruction{
        address: u32
    },
    WatchRangeInstruction{
        range:[u32;2],
        exclude:Vec<u32>
    }

}
