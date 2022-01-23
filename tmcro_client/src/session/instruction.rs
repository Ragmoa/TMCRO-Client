//Orders that can be sent to the bridge
use json::JsonValue;
use json::number::Number;
use log::error;


pub enum Instruction{
    OrInstruction{
        address: u32,
        value: u8,
    },
    AddInstruction{
        startaddress: u32,
        value: u32,
        max: u64,
        length: u8
    },
    WatchByteInstruction{
        address: u32
    },
    WatchRangeInstruction{
        range:[u32;2],
        exclude:Vec<u32>
    },
    MessageInstruction{
        message:String,
        color:String
    }
}
impl Instruction{


    pub fn to_json(&self) -> Result<String,String> {
        let mut json_data=JsonValue::new_object();
        match self {
            Instruction::OrInstruction{address,value} => {
                json_data["order"]=json::JsonValue::String("OR".to_string()).into();
                json_data["address"]=Number::from_parts(true,*address as u64 ,0).into();
                json_data["value"]=Number::from_parts(true,*value as u64 ,0).into();
                Ok(json::stringify(json_data))
            }
            Instruction::AddInstruction{startaddress,value,max,length} => {
                json_data["order"]=json::JsonValue::String("ADD".to_string()).into();
                json_data["startAddress"]=Number::from_parts(true,*startaddress as u64 ,0).into();
                json_data["value"]=Number::from_parts(true,*value as u64 ,0).into();
                json_data["max"]=Number::from_parts(true,*max as u64 ,0).into();   
                json_data["length"]=Number::from_parts(true,*length as u64 ,0).into();
                Ok(json::stringify(json_data))
            }
            Instruction::WatchByteInstruction {address} => {
                json_data["order"]=json::JsonValue::String("WATCH".to_string()).into();
                json_data["address"]=Number::from_parts(true,*address as u64 ,0).into();
                Ok(json::stringify(json_data))
            }
            Instruction::WatchRangeInstruction {range,exclude} => {
                if (range[0] >= range[1]){
                    let error=String::from("Invalid WATCH range instruction created!, range starts at ") + &range[0].to_string() + " and ends at " + &range[1].to_string();
                    Err( error )
                } else {
                json_data["order"]=json::JsonValue::String("WATCH".to_string()).into();
                let mut range_vec:Vec<JsonValue>=Vec::with_capacity(2);
                for rvalue in range{
                    range_vec.push(Number::from_parts(true, *rvalue as u64 ,0).into());
                }
                json_data["range"]=json::JsonValue::Array(range_vec);
                let mut exclude_vec:Vec<JsonValue>=Vec::with_capacity(exclude.len());
                for evalue in exclude{
                    exclude_vec.push(Number::from_parts(true, *evalue as u64 ,0).into());
                }
                json_data["exclude"]=json::JsonValue::Array(exclude_vec);
                Ok(json::stringify(json_data))
            }
            }
            Instruction::MessageInstruction{message,color} => {
                json_data["order"]=json::JsonValue::String("MSG".to_string()).into();
                json_data["color"]=json::JsonValue::String(color.to_string()).into();
                json_data["message"]=json::JsonValue::String(message.to_string()).into();
                Ok(json::stringify(json_data))
            }
        }
    }


}
