# getcustomasset
A simple getcustomasset fix written in rust, mainly for executors like awp or others where getcustomaseet is currentyl broken

The build needs to be run as admin

dependencies
```toml
tokio = { version = "1.44", features = ["full"] }
tokio-tungstenite = "0.21"
tungstenite = "0.21"
whoami = "1.6.0"
reqwest = { version = "0.12", features = ["json"] }
futures = "0.3"
futures-util = "0.3.31"
```

script (put this in autoexc)
```luau
local ws = WebSocket.connect("ws://127.0.0.1:6969")

local latest_asset;
ws.OnMessage:Connect(function(msg)  
    latest_asset = msg
end)

getgenv().getcustomasset = function(path)
    ws:Send(path)
    repeat task.wait() until latest_asset ~= nil

    local asset = latest_asset
    latest_asset = nil  

    return asset
end 
```
