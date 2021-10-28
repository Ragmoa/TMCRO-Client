//Orders that can be sent to the bridge
use json::JsonValue;
use json::number::Number;

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
impl Instruction{

    pub fn to_json(&self) -> String {
        let mut json_data=JsonValue::new_object();
        match self {
            Instruction::WriteInstruction{address,value} =>{
                json_data["order"]=json::JsonValue::String("WRITE".to_string()).into();
                json_data["address"]=Number::from_parts(true,*address as u64 ,0).into();
                json_data["value"]=Number::from_parts(true,*value as u64 ,0).into();
                json::stringify(json_data)
            }
            Instruction::WatchByteInstruction {address} => {
                json_data["order"]=json::JsonValue::String("WATCH".to_string()).into();
                json_data["address"]=Number::from_parts(true,*address as u64 ,0).into();
                json::stringify(json_data)
            }
            Instruction::WatchRangeInstruction {range,exclude} => {
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
                json::stringify(json_data)
            }
        }
    }


}
