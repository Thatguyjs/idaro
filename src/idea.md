# Architecture (run)

```
start webserver at [web_addr]
start wsserver at [ws_addr]

on ctrl+c:
  webserver.exit();
  wsserver.exit();
  log exit message
```


## WebServer
A single-threaded webserver for realtime markdown parsing & file hosting

### bind(addr: Addr) -> Result\<Self, io::Error\>
Create a new WebServer bound to [addr]

### listen() -> Result<(), io::Error>
Non-blocking, start a separate thread for a Socket to wait for connections

### shutdown() -> Result<(), io::Error>
Stop listening for connections

### wait()
Block until shutdown() is called


## WsServer
A single-threaded WebSocket server, used for live-updating pages

### listen(addr: Addr) -> Self
Non-blocking, start a separate thread for a Socket to wait for connections

### shutdown() -> Result<()>
Stop listening for connections
