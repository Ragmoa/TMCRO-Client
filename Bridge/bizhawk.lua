mrb=memory.readbyte
mwb=memory.writebyte
local status="INIT"
local json = require('json')
local socket = require('socket.core')
local address = "127.0.0.1"
local port = 43884
local tcp = nil
local textDisplay={}
local framecount=0

function checkConnection()
  if comm.socketServerIsConnected() == true then
    status="CONNECTED"
  else
    status="DISCONNECTED"
  end
end

function addText(color,text)
  local element={
    ["timer"]=300,
    ["color"]=color;
    ["text"]=text;
  }
  table.insert(textDisplay,element)
end

function clientConnect()
  tcp = socket.tcp()
  local ret, err = tcp:connect(address, port)
  if ret == 1 then
    addText("green","Connected to client")
    tcp:settimeout(0)
    tcp:setoption('keepalive',true)
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
    if text['timer']==0 then
      table.remove(textDisplay,key)
    end
    text['timer']=text["timer"]-1
    gui.text(0,height,text['text'],text['color'],"bottomright")
    height=height+15
  end
end





-- -- Initial connection
-- while comm.socketServerIsConnected() === false do
--   gui.text(0,0,"Not connected to Client","red","black","bottomright")
--   clientConnect()
--   emu.frameadvance()
-- end

--Connected, time for main loop
while true do
  if tcp==nil then
    clientConnect()
  else
      ret,err=tcp:send("HELLO")
      if ret then
        print(ret)
      else
        print (err)
      end
    local data, err, part = tcp:receive('*l')
    if data and #data then
      print(data)
    end
  end
--   checkConnection()
--   console.log("check done!")
--   if status == "CONNECTED" then
--     emu.frameadvance()
--   end
--   if status == "DISCONNECTED" then
--     gui.text(0,0,"Disconnected from Client","red","black","bottomright")
--   end
  displayText()
  emu.frameadvance()
end
