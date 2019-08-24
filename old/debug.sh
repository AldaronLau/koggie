# Install new version
cargo build && sudo cp target/debug/koggie /usr/bin/koggie
# Copy config folder
sudo cp -rT config /usr/share/koggie
sudo chmod -R a+rwX /usr/share/koggie
