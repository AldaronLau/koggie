# Koggie
Ogg streaming broadcast &amp; listen application.

# Getting Started
## Install Koggie Webserver
What you need:
- cargo & rustc (to build Koggie)
- koggie (the webserver).
- systemfd (to securely give port 80 to Koggie)

```
# Koggie is coded in Rust, so you'll need that first.
curl https://sh.rustup.rs -sSf | sh

# Install systemfd to securely give koggie permission to host on port 80
cargo install systemfd --force
sudo setcap 'cap_net_bind_service=+ep' `which systemfd`

# Get Koggie
git clone git@github.com:kaugradio/koggie.git
cd koggie

# Install koggie (may need sudo)
./install
```

## Run Server
Note: Change `aldaron` to your username before running.

```
systemfd --no-pid -s http::10.0.0.159:80 -- koggie /home/aldaron/.koggie/ 10.0.0.159
```

## Edit Server
Koggie stores all of your static files in `res/`.  `res/` contains a few HTML fragments and markdown
for each page.  Audio files are stored in whatever location is passed to koggie on the command line.
