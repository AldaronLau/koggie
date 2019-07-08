# Koggie
Ogg streaming broadcast &amp; listen webserver.

# Getting Started
Koggie is coded in Rust, and uses cargo, so you'll need that first.  Note: Koggie needs to be recompiled specifically for your project each time a page changes (but don't worry, it's easy!😉️).

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
Note: Change `aldaron` to your username before running.

```
cd /home/aldaron/Projects/koggie/
systemfd --no-pid -s http::10.0.0.159:80 -- cargo watch -x 'run --release -- /home/aldaron/.koggie/ 10.0.0.159'
```

## Edit Server
Koggie stores all of your static files in `res/`.  `res/` contains a few HTML fragments and markdown
for each page.  Audio files are stored in whatever location is passed to koggie on the command line.
