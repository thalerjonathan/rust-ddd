# only run single threaded as otherwise we gonna run into data races in DB
cargo test --release -- --test-threads=1