bigprimes.net
======

Serverless Rust with Web Assmebly.

[![Rust](https://img.shields.io/badge/Rust%20%3E%3D%201.36-000.svg?style=flat-square&logo=rust&colorA=ffffff&style=popout)](https://rust-lang.org/)
[![Build Status](https://github.com/craigmayhew/bigprimes.net/workflows/Rust/badge.svg)](https://github.com/craigmayhew/bigprimes.net/actions)

Setup your local dev environment
===

 1. Clone bigprimes.net `git clone https://github.com/craigmayhew/bigprimes.net.git`
 2. Install cargo rustc, rustup `curl https://sh.rustup.rs -sSf | sh`
 3. Install wasm-pack `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
 4. `cargo install --locked trunk`
 5. `cargo install wasm-bindgen-cli`
 6. Build `trunk build`
 7. Build, serve and rebuild on changes `trunk serve --open`

Contributing
===

 1. Write code
 2. Run tests
 3. Create PR

Build
===

 - Build: `wasm-pack build` or `trunk build`
 - Build and serve locally: `trunk serve --open`
 
Fun Facts
===
- there are 1.4\*10<sup>297</sup> primes smaller than 300 digits
- there is always a prime between n^2 and (n+1)^2.

Thanks
===
 - https://seed-rs.org/
 - https://github.com/thedodd/trunk
 
