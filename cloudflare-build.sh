curl https://sh.rustup.rs -sSf | sh -s -- -y
. $HOME/.cargo/env
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -y
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
cargo install wasm-bindgen-cli
trunk build
