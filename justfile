create folder:
    @mkdir -p {{ folder }} && cp -r template/* {{ folder }}

run day file:
    cd {{ day }} && cargo run --bin {{ file }}

test day file:
    cd {{ day }} && cargo test --bin {{ file }}
