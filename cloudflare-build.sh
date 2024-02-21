curl https://sh.rustup.rs -sSf | sh -s -- -y
. $HOME/.cargo/env
rustup default nightly
rustup target add wasm32-unknown-unknown
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -y
cargo +stable install --locked trunk
cargo +stable install wasm-bindgen-cli
trunk build --release
