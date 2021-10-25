//Orders that can be sent to the bridge
use json::JsonValue;

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

    fn writeJson(&self) -> JsonValue{
        let mut jsonData=JsonValue::new_object();
        jsonData["order"] = "WRITE".into();
        jsonData["adress"]= self.address.into();
        jsonData["value"]=self.value.into();
        jsonData
    }

    fn watchJson(&self) -> JsonValue{
    }
    fn toJson(&self) -> String{

        match self{
            Instruction::WriteInstruction => self.writeJson()

            Instruction::WatchByteInstruction =>
                jsonData["order"] = "WATCH".into(),
                jsonData["adress"]= self.address.into(),

            Instruction::WatchRangeInstruction =>
                jsonData["order"] = "WATCH".into(),
                jsonData["range"]= self.range.into(),
                jsonData["exclude"]=self.exclude.into(),

        }
        json::stringify(jsonData)
    }

}
