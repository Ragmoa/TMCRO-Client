mod normalitem;
mod itemorigin;
mod itemchain;
mod inventorycounter;

use crate::inventory::normalitem::NormalItem as NormalItem;
use crate::inventory::itemchain::ItemChain as ItemChain;
use crate::inventory::inventorycounter::InventoryCounter as InventoryCounter;


pub struct Inventory {
    normalitems: Vec<NormalItem>,
    progressiveitems: Vec<ItemChain>,
    counters: Vec<InventoryCounter>
}