# HTTP Server in Rust

## How to Run

1. Edit the `settings.toml` file with the correct IP, port, etc.

2. Run `cargo run --release`

## Settings.toml

 - ip: A string containing the IP to bind to. Example: `"10.0.3.54"`
 - port: An integer containing the port to bind to. If the port is below 1024, you will need admin rights. Example: `80`
 - threads: The amount of threads to deploy to handle incoming requests. Example: `5`
 - path: The folder which will act as the root file system for the web server. Example: `"./site"`
 - teapot: OPTIONAL. Replaces all 404 errors with 418 I'm a teapot. Example: `true`

For reference, here is an example `settings.toml` file:

```toml
ip = "127.0.0.1"
port = 8000
threads = 5
path = "./site"
```