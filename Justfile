nixdev := "nix develop -c"
opts := "-- --path \"/mnt/NAS/Anime/Konosuba/Konosuba Season 1/\" --season 1 --dryrun"

fast:
        {{nixdev}} cargo run --release {{opts}}
slow:
        {{nixdev}} cargo run {{opts}}

build:
        {{nixdev}} cargo build --release
bench:
        {{nixdev}} cargo bench
test:
        {{nixdev}} cargo test
