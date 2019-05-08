# Koggie
Ogg streaming broadcast &amp; listen application.

# Setting Up
First, we'll set up the network so we can run a web server:

```bash
sudo iptables -t nat -A PREROUTING -p tcp --dport 80 -j REDIRECT --to-port 8080
sudo iptables -t nat -I OUTPUT -p tcp -d 127.0.0.1 --dport 80 -j REDIRECT --to-ports 8080
```

Install dependencies.

`cargo install systemfd cargo-watch --force`

## Install
```
./install.sh
```

## Run Koggie
```
sudo $HOME/.cargo/bin/koggie 10.0.0.159
```
