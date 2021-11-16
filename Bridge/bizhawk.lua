mrb=memory.readbyte
mwb=memory.writebyte
local status="INIT"
local json = require('json')
local socket = require('socket.core')
local address = "127.0.0.1"
local port = 65398
local tcp = nil
local textDisplay={}
local framecount=0
local watchTable={}
local notificationQueue={}
-- How many bytes we're checking per frame
local bytesWatchedPerFrame=5
local watchCounter=0

function table.contains_key(table, element)
  for value, _ in pairs(table) do
    if value == element then
      return true
    end
  end
  return false
end

function table.contains_value(table, element)
  for _, value in pairs(table) do
    if value == element then
      return true
    end
  end
  return false
end



function addWatch(address)
  local watchElement={
    ['address']=address;
    ['value']=nil
  }
  table.insert(watchTable,watchElement)
end

function doWatches()
  for i=watchCounter, watchCounter+bytesWatchedPerFrame, 1
  do
    local tableMax=table.maxn(watchTable)
    -- print(watchTable[index])
    local index=nil
    if tableMax>0 then
      -- Arrays starts at 1 with this hecking language...
      local index=(i%table.maxn(watchTable))+1
      local byte=memory.readbyte(watchTable[index]['address'])
      if watchTable[index]['value']~=byte then
          watchTable[index]['value']=byte
          addWatchNotification(watchTable[index]['address'],byte)
          addText("yellow",byte)
        end
      end
    end
end

function addWatchNotification(address,value)
  local element={
    ["type"]="WATCH";
    ["address"]=address;
    ["value"]=value;
  }
  table.insert(notificationQueue,element)
end

-- sends the Notification at top of the queue
function sendNotification()
  if (table.maxn(notificationQueue)>0)
  then
    local element=notificationQueue[1]
    local jsonElement=json.encode(element)
    local ret, err=tcp:send(jsonElement.."\n")
    if (err==nil) then
      table.remove(notificationQueue,1)
    end
  end
end

function addText(color,text)
  local element={
    ["timer"]=300;
    ["color"]=color;
    ["text"]=text;
  }
  table.insert(textDisplay,1,element)
end

function handle_instruction(data)
  local instruction=json.decode(data)
  if instruction['order']=='WATCH'
  then
    if table.contains_key(instruction, 'address')
    then
      addWatch(instruction['address'])
    end
    if (table.contains_key(instruction, 'range') and table.contains_key(instruction, 'exclude'))
    then
       for (i=instruction['range'][1],instruction['range'][2],1)
       do
         if table.contains_value(instruction['exclude'],i)
         then
         else
           addWatch(i)
         end
    end
  end
  if instruction['order']=="MSG"
  then
    if (table.contains_key(instruction,"message") and table.contains_key(instruction,"color"))
    then
      addText(instruction['message'],instruction['color'])
    end
  end
end

function clientConnect()
  tcp = socket.tcp()
  local ret, err = tcp:connect(address, port)
  if ret == 1 then
    addText("green","Connected to client")
    tcp:settimeout(0)
    tcp:setoption('keepalive',true)
    ret,err=tcp:send("HELLO\n")
  else
    addText("red","Failed to connect to client")
    print('Failed to open socket:', err)
    tcp:shutdown()
    tcp:close()
    tcp = nil
  end
end

function displayText()
  height=5;
  for key,text in pairs(textDisplay) do
    if text['timer']<=0 then
      table.remove(textDisplay,key)
    end
    text['timer']=text["timer"]-1
    gui.text(0,height,text['text'],text['color'],"bottomright")
    height=height+15
  end
end

-- addWatch(0x02002AEA)
-- Main Loop
while true do
  if tcp==nil then
    clientConnect()
  else
    local data, err, part = tcp:receive('*l')
    if data and #data then
      print(data)
      handle_instruction(data);
    end
    if err and #err and err~="timeout" then
      print(err)
    end
  end
  displayText()
  doWatches()
  sendNotification()
  emu.frameadvance()
end
