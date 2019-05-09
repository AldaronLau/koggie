# Koggie
Ogg streaming broadcast &amp; listen application.

# Getting Started
Koggie is coded in Rust, and uses cargo, so you'll need that first.  Note: Koggie needs to be recompiled specifically for your project.

```
curl https://sh.rustup.rs -sSf | sh
```

## Install Server
First, we need to install some Rust CLI programs & securely give it permission to host on port 80.

```
cargo install systemfd cargo-watch --force
sudo setcap 'cap_net_bind_service=+ep' `which systemfd`
```

## Run Server
```
cd /home/aldaron/Projects/koggie/
systemfd --no-pid -s http::10.0.0.159:80 -- cargo watch -x 'run --release -- /home/aldaron/Projects/koggie/.koggie/ 10.0.0.159'
```
