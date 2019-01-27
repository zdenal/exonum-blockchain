## Why
Get familiar with Exonum blockchain principles. Split logic into
meaningfull layers and modules for better future evolving.

## Purpose
Make more services (currency, vote, ...) for variety blockchain use (like smart contracts
in some other blockchain platforms).

## Run
- execute `cargo run --example node`
- run REST requests placed in: `examples/rest_requests.rest`
- run transactions created via javascript by `node node/index.js`

## TODO
[ ] - voting service - split `post_transaction` endpoint handler into 2 separated. One for create candidate and
one for create vote (just for playing reason).
