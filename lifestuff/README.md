# Lifestuff

A Rust-based command-line utility for lifestyle calculations and conversions, including unit conversions, date operations, financial calculations, currency conversions, mileage tracking, and DuckDuckGo email utilities.

## Building

To build simply run `cargo build` in the root directory. For a quicker verification run `cargo check`

**Note**: Building automatically generates Zsh shell completions in the build output directory.

## Installing

To install simply run `cargo install --path .` from the `lifestuff/` directory.

## Shell Completions

Zsh completions are automatically generated during the build process using the `build.rs` script. The completion file `_lifestuff` is placed in:
- `$RUST_COMPLETION_DIR` (if set)
- Or `$OUT_DIR/completion/` (build target directory)

To use completions, copy the generated `_lifestuff` file to a directory in your `$fpath`.

## Running

The program provides various utilities for lifestyle calculations and conversions.

### Global Options

- `--verbose` (`-v`): Enable detailed output for all commands
- `--help` (`-h`): Show help for any command or subcommand
- `--version` (`-V`): Display version information

```
Something Something Something Daaaarksiiiide

Usage: lifestuff [OPTIONS] <COMMAND>

Commands:
  convert   Unit conversions
  dates     Date Operations
  interest  Interest Calculations
  currency  Currency Conversion Operations
  mileage   Mileage Calculations
  ddg       DuckDuckGo Address
  help      Print this message or the help of the given subcommand(s)

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

Comprehensive mortgage and loan calculations with support for:
- Principal tracking
- Interest rate calculations  
- Monthly payment planning
- Annual repayment limits (percentage or fixed amount)
- End date projections

```
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
```

### Currency

Real-time currency conversion using external APIs. Supports converting to multiple target currencies in a single command.

```
Currency Conversion Operations

Usage: lifestuff currency [OPTIONS] --from <FROM> --amt <AMT>

Options:
  -f, --from <FROM>  Currency to convert from
  -v, --verbose
  -a, --amt <AMT>    Amount to convert
  -t, --to <TO>      Currency to convert to (can be specified multiple times)
  -h, --help         Print help

```

**Example**: `lifestuff currency --from USD --amt 100 --to EUR --to GBP --to JPY`

### Mileage

Track current vehicle mileage against projected values. Useful for lease agreements and maintenance scheduling.

```
Mileage Calculations

Usage: lifestuff mileage [OPTIONS] --mileage <MILEAGE>

Options:
  -m, --mileage <MILEAGE>  Current mileage of the vehicle
  -v, --verbose
  -h, --help               Print help

```

### DuckDuckGo (DDG)

Utilities for managing DuckDuckGo email aliases and forwarding.

```
DuckDuckGo Address

Usage: lifestuff ddg [OPTIONS] <COMMAND>

Commands:
  generate  Generates a Duckduckgo email alias address
  convert   Converts a regular email address to be used by Duckduckgo as a recipient
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  
  -h, --help     Print help
```

## Project Structure

This is a Rust workspace with two crates:
- `lifestuff/`: Main binary crate with command implementations
- `lifestuff-types/`: Shared types and CLI argument definitions

## Dependencies

- **clap**: Command-line argument parsing
- **anyhow**: Error handling
- **time**: Date and time operations
- **reqwest**: HTTP client for currency API calls
- **serde**: JSON serialization
- **colored**: Terminal output formatting

For a complete technical overview, see `CLAUDE.md`.
