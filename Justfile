nixdev := "nix develop -c"
opts := "-- --path \"/mnt/NAS/Anime/Konosuba/Konosuba Season 1/\" --season 1 --dryrun"
hypr := "--path ./test_files/ --season 69"
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
hyper:
        hyperfine './target/release/autorenamer {{hypr}}' './target/debug/autorenamer {{hypr}}'  --prepare ./benchmarks.sh --runs 5
