
use crate::inventory::itemorigin::ItemOrigin as ItemOrigin;



pub struct NormalItem{
    name: String,
    address: u32,
    value: u8,
    origin: ItemOrigin
}