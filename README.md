bigprimes.net
======

Serverless Rust with Web Assmebly.

[![Rust](https://img.shields.io/badge/Rust%20%3E%3D%201.36-000.svg?style=flat-square&logo=rust&colorA=ffffff&style=popout)](https://rust-lang.org/)
[![Codecov branch](https://img.shields.io/codecov/c/github/craigmayhew/bigprimes.net/master.svg)](https://codecov.io/gh/craigmayhew/bigprimes.net)
[![Build Status](https://travis-ci.org/craigmayhew/bigprimes.net.svg?branch=master)](https://travis-ci.org/craigmayhew/bigprimes.net)

Setup your local dev environment
===

 1. Install cargo rustc, rustup `curl https://sh.rustup.rs -sSf | sh`
 2. Install wasm-pack `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
 3. If developing on Ubuntu, `apt-get install libssl-dev pkg-config`
 4. Use a cargo template `cargo install cargo-generate`
 5. Install npm `npm install npm@latest -g`

Setup project for the first time
===
You almost certainly do not need to do these!
 1. `npm init wasm-app www`
 2. `cd www && npm install && npm audit fix`

Contributing
===

 1. Write code
 2. Run tests
 3. Create PR

Build
===

 1. Compile rust into wasm `wasm-pack build`
 2. Build web app `cd www && npm install`
 3. 
 
Deploy
===

 -
 -

Fun Facts
===
- there are 1.4\*10<sup>297</sup> primes smaller than 300 digits
- there is always a prime between n^2 and (n+1)^2.

Thanks
===
 - https://seed-rs.org/
 - https://tatrix.org/public/html-to-seed/
 