# Install new version
cargo build --release && sudo cp target/release/koggie /usr/bin/koggie
# Copy config folder
sudo cp -ruT config /usr/share/koggie
sudo chmod -R a+rwX /usr/share/koggie
