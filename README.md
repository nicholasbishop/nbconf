# nbconf

[![crates.io](https://img.shields.io/crates/v/nbconf.svg)](https://crates.io/crates/nbconf)
[![Documentation](https://docs.rs/nbconf/badge.svg)](https://docs.rs/nbconf)

Simple configuration file reader/writer. This is intended for use with
config files that are human readable but machine written.

## Format

The format is simple: key-value pairs nested under sections. Example:

```
[Section 1]
hello = world

[Section 2]
nice to = meet you
```
