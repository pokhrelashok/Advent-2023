create folder:
    @mkdir -p {{ folder }} && cp -r template/* {{ folder }}

run day file:
   cargo run --package {{ day }} --bin {{ file }}

test day file:
   cargo test --package {{ day }} --lib -- {{ file }}::tests
