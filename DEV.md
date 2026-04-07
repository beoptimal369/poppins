## How to run all tests?
- `cargo test`


## How to run tests and see println!'s?
- `cargo test -- --nocapture`


## How to run tests for one file?
- `cargo test cli_command::tests`


## How to run one test?
- `cargo test cli_command::tests::test_missing_model_name`


## How to create test coverage report?
1. `cargo install cargo-tarpaulin`
2. `cargo tarpaulin --out Html --out Json`


## How to get test badge
- `bash <(curl -s https://codecov.io/bash) -t <token> -f tarpaulin-report.json`
