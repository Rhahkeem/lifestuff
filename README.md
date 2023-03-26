# Lifestuff

## Building

To build simply run `cargo build` in the root directory

## Installing

To install simple run `cargo install --path .`

## Running

The program has some options to assist with conversions.

```
/lifestuff --help

Something Something Something Daaaarksiiiide

Usage: lifestuff [OPTIONS] <COMMAND>

Commands:
  convert
          Unit conversions
  dates
          Date Operations
  help
          Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose


  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### Unit Conversions

This allows for conversion between different units :

```
Unit conversions

Usage: lifestuff convert [OPTIONS] <COMMAND>

Commands:
  area      Area Conversions
  distance  Distance Conversions
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
  -h, --help     Print help
  ```

For example (as seen above) it can perform conversions between distance units, and area units.

### Date Operations

This allows various date operations such as diffs and adding time periods to a given date

```
Date Operations

Usage: lifestuff dates [OPTIONS] <COMMAND>

Commands:
  add   Add a time period to a given date
  diff  Diff Two Dates
  help  Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
  -h, --help     Print help
  ```