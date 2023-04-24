# Lifestuff

## Building

To build simply run `cargo build` in the root directory. For a quicker verification run `cargo check`

## Installing

To install simple run `cargo install --path .`

## Running

The program has some options to assist with conversions.

```
Something Something Something Daaaarksiiiide

Usage: lifestuff [OPTIONS] <COMMAND>

Commands:
  convert
          Unit conversions
  dates
          Date Operations
  interest
          Interest Calculations
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

### Interest

Interest Calculations

Usage: lifestuff interest [OPTIONS] --principal <PRINCIPAL> --interest-rate <INTEREST_RATE> --repayment <REPAYMENT> --max-repayment-pct <MAX_REPAYMENT_PCT> --annual-downpayment <ANNUAL_DOWNPAYMENT> --end-date <END_DATE>

Options:
  -p, --principal <PRINCIPAL>
          Principal left on mortgage
  -v, --verbose

  -i, --interest-rate <INTEREST_RATE>
          Interest rate (%)
      --repayment <REPAYMENT>
          Monthly payment amount
  -m, --max-repayment-pct <MAX_REPAYMENT_PCT>
          Max annual repayment percentage (%) [aliases: annual-limit]
  -a, --annual-downpayment <ANNUAL_DOWNPAYMENT>
          Max annual supplemntary downpayment  [aliases: annual-downpayment]
  -e, --end-date <END_DATE>
          Mortgage calculation end date (dd/mm/yyyy)
  -h, --help
          Print help