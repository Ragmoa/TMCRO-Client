mrb=memory.readbyte
mwb=memory.writebyte
local status="INIT"
local json = require('json')
local socket = require('socket.core')
local address = "127.0.0.1"
local port = 43884
local tcp = nil

function checkConnection()
  if comm.socketServerIsConnected() == true then
    status="CONNECTED"
  else
    status="DISCONNECTED"
  end
end

function clientConnect()
  tcp = socket.tcp()

  local ret, err = tcp:connect(address, port)
  if ret == 1 then
    gui.text(10,10,'Connection established',"blue","bottomright")
    tcp:settimeout(0)
    print ('ok')
  else
    print('Failed to open socket:', err)
    tcp:shutdown()
    tcp:close()
    tcp = nil
  end
end

-- -- Initial connection
-- while comm.socketServerIsConnected() === false do
--   gui.text(0,0,"Not connected to Client","red","black","bottomright")
--   clientConnect()
--   emu.frameadvance()
-- end

--Connected, time for main loop
clientConnect()
while true do
  if tcp then
    local data, err, part = tcp:receive('*l')
    print(data)
    if data == nil then
      print(err)
      tcp=nil
      print ("Nothing found")
  else
    clientConnect()
  end
end
--   checkConnection()
--   console.log("check done!")
--   if status == "CONNECTED" then
--     emu.frameadvance()
--   end
--   if status == "DISCONNECTED" then
--     gui.text(0,0,"Disconnected from Client","red","black","bottomright")
    emu.frameadvance()
--   end
end
