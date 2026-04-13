# eztamp_logger

EZTamp Logger is a very lightweight low dependency logging solution with colored console logging, and tamper evident rolling hashes for messages in output file that can be verified to ensure data integrity of log file.

## Environment Variables

Optional environment variables, these can be loaded into your deployment pipeline in any matter you desire, and all have default values if not present.

```bash
# Log level, can be expressed as TRACE | DEBUG | INFO | WARN | ERROR | FATAL | NONE or as a u16.
# Log level u16 equivalents are 500 | 400 | 300 | 200 | 100 | 1 | 0 if compared to the list above
# Custom log calls exist with whatever value you desire that fits within a u16 for filtering
RUST_LOG=TRACE # default = INFO
# This is the secret used to hash the rolling checksums within the log file
RUST_LOG_SECRET=THISISABADSECRET # default = ""
# Output path for logging
RUST_LOG_OUTPUT_FILE="./log.txt" # default = "./log.txt"
# Bitmask that controls destination for log calls
# Flags
# File = 1000
# StdOut = 0100
# StdErr = 0010
# Default = 1001
# By default file logging is enabled, and Fatal and Error messages are piped to stderr with all other levels piping to stdout
RUST_LOG_DESTINATION=1001 # default = 1001
```

## Validation

To validate log file, run the validation test solution in the tests.rs file. This will load output file from dotenv and will read from a local .env file.

## Contribute

Feel free to take a look at the current issues in this repo for anything that currently needs to be worked on.

You are also welcome to open a PR or a new issue if you see something is missing or could be improved upon.

## License

MIT