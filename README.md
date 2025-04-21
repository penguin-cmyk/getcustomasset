# getcustomasset
A simple getcustomasset fix written in rust, mainly for executors like awp or others where getcustomaseet is currentyl broken

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
