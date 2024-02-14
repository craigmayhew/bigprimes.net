curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -y
rustup target add wasm32-unknown-unknown
cargo install cargo-binstall
cargo-binstall --locked --no-confirm trunk
cargo install wasm-bindgen-cli
rustup default nightly
trunk build
