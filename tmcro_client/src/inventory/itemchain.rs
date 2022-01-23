use crate::memory::memory;

pub enum ItemChainType{
    SwordChainType,
    BombsChainType,
    BombBagChainType,
}

pub struct ItemChainTypeLevel{
    name:String,
    address:u32,
    value:u8,
}

pub struct ItemChain{
    // use signed number so we can set it to -1 when the player doesn't have anything in the chain
    level: i8,
    levels: Vec<ItemChainTypeLevel>,
}

impl ItemChain{
    fn init(itemtype:ItemChainType) -> ItemChain{
        let mut chain= ItemChain{level:-1,levels:vec![]};
        match itemtype{
            SwordChainType => {
                chain.levels.push(ItemChainTypeLevel{name:"Smith's sword".to_string(),address:memory::ewram_address_unsafe(0x2B32),value:4});
                chain.levels.push( ItemChainTypeLevel{name:"White sword".to_string(),address:memory::ewram_address_unsafe(0x2B32),value:10});
                chain.levels.push( ItemChainTypeLevel{name:"White sword (2)".to_string(),address:memory::ewram_address_unsafe(0x2B32),value:40});
                chain.levels.push( ItemChainTypeLevel{name:"White sword (3)".to_string(),address:memory::ewram_address_unsafe(0x2B33),value:1});
                chain.levels.push( ItemChainTypeLevel{name:"Unused sword".to_string(),address:memory::ewram_address_unsafe(0x2B33),value:4});
                chain.levels.push(ItemChainTypeLevel{name:"Four Sword".to_string(),address:memory::ewram_address_unsafe(0x2B33),value:10});
            }
        }

        chain
    }
}