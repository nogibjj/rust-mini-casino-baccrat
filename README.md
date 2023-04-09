# Unix Timestamp CLI Tool
This is a simple command-line tool for working with Unix timestamps. It can be used to print the current Unix timestamp and time, or to calculate the time difference between a given Unix timestamp and the current time.

## What is a Unix timestamp?
A Unix timestamp, also known as POSIX time, is a system for representing dates and times as a single integer, representing the number of seconds that have elapsed since midnight on January 1, 1970 UTC. This system is widely used in computer systems and programming languages as a standard way to represent dates and times, and is supported by most operating systems and programming environments.

## Usage
The tool can be run with the cargo run command, followed by one of the following subcommands:

`-c`: Print the current Unix timestamp and time.
`-d -u <unix_timestamp>`: Calculate the time difference between the given Unix timestamp and the current time. The time difference can be displayed either in Unix seconds (with the `-u` option), or as a formatted string (default behavior).

## Examples
```
# Print the current Unix timestamp and time
cargo run -c

# Calculate the time difference between a given Unix timestamp and the current time
cargo run -d -u 1617979423
cargo run -d 1617979423 # same as above, default behavior
cargo run -d -u abc # invalid Unix timestamp
```

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
