pub mod memory{

    pub fn ewram_address_unsafe(address:u32)->u32{
            0x2000000+address
    }
        
    pub fn ewram_address(address:u32)-> Result<u32,String>{
        if (address > 0x3FFFF){
            Err("Wrong range for a EWRAM address:".to_string())
        } else {
            Ok(0x2000000+address)
        }
    }

    pub fn iwram_address_unsafe(address:u32)->u32{
        0x3000000+address
    }
    pub fn iwram_address(address:u32)->Result<u32,String>{
        if (address > 0x7FFF){
            Err("Wrong range for a IWRAM address:".to_string())
        } else {
            Ok(0x3000000+address)
        }
    }

}