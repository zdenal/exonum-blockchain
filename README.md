## Why
Get familiar with Exonum blockchain principles. Split logic into
meaningfull layers and modules for better future evolving.

## Purpose
Make more services (currency, vote, ...) for variety blockchain use (like smart contracts
in some other blockchain platforms).

## Run
- execute `cargo run`
- run REST requests placed in: `rest_requests/wallets.rest`

## TODO
[ ] - better using SERVICE_ID ... only declaring in service and reuse in another places (eg. transactions)
[ ] - voting service - split `post_transaction` endpoint handler into 2 separated. One for create candidate and
one for create vote (just for playing reason).
[ ] - make schema queries reproduce directly errors not options. Eg. `schema.candidate(pub_key)` should return directly error CandidateNotFound.
This could simplified logic in transactions/service itself.
