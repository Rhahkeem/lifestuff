# Lifestuff

A Rust-based command-line utility for lifestyle calculations and conversions, including unit conversions, date operations, financial calculations, currency conversions, mileage tracking, mortgage management, and DuckDuckGo email utilities.

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
  mortgage  Mortgage Management
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
  -f, --from <FROM>        Currency to convert from
  -v, --verbose
  -a, --amt <AMT>          Amount to convert
  -t, --to <TO>            Currency to convert to (can be specified multiple times)
      --endpoint <ENDPOINT> Override the currency API endpoint URL
  -h, --help               Print help

```

**Examples**: 
```bash
# Basic currency conversion
lifestuff currency --from USD --amt 100 --to EUR --to GBP --to JPY

# Override the API endpoint for a one-off command
lifestuff currency --endpoint https://example.com --from USD --amt 100 --to EUR
```

**Configuration**:
- Default host: `http://localhost:8787` (local development)
- Override per command with `--endpoint <URL>`
- Set `LIFESTUFF_API_ENDPOINT` for a persistent override (takes effect when `--endpoint` is omitted)
- All monetary values in respective currency units

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

### Mortgage

Comprehensive mortgage tracking and management system that integrates with a Cloudflare Workers backend. Track payments, view status, and monitor interest postings.

```
Mortgage Management

Usage: lifestuff mortgage [OPTIONS] <COMMAND>

Commands:
  payment           Record a mortgage payment
  status            Get current mortgage status
  history           Get payment history
  interest-history  Get interest posting history
  interest          Add an interest posting
  help              Print this message or the help of the given subcommand(s)

Options:
      --endpoint <ENDPOINT>            Override the mortgage API endpoint URL
  -v, --verbose
  -h, --help                           Print help
```

**Payment Command Options**:

The `payment` subcommand supports flexible payment recording:

- `--default` / `-D`: Use mortgage's default monthly payment
- `--monthly` / `-m`: Specify custom monthly payment amount (conflicts with `--default`)
- `--overpayment` / `-o`: Add overpayment on top of monthly payment
- If neither `--default` nor `--monthly` is specified, monthly payment defaults to 0 (pure overpayment)

**Examples**:

```bash
# Record default monthly payment
lifestuff mortgage payment --default --date "01/02/2024"

# Record default monthly payment with overpayment
lifestuff mortgage payment --default --overpayment 500 --date "01/02/2024"

# Record overpayment only (no monthly payment)
lifestuff mortgage payment --overpayment 500 --date "01/02/2024"

# Record custom monthly payment with overpayment
lifestuff mortgage payment --monthly 1200 --overpayment 500 --date "01/02/2024" \
  --note "Extra payment"

# Record custom monthly payment only
lifestuff mortgage payment --monthly 1200 --date "01/02/2024"

# Check mortgage status
lifestuff mortgage status

# View payment history
lifestuff mortgage history

# View interest history
lifestuff mortgage interest-history

# Add an interest posting
lifestuff mortgage interest --amount 50 --date "01/01/2024" \
  --note "Monthly interest"

# Override the API endpoint for a one-off command
lifestuff mortgage --endpoint https://example.com status
```

**Configuration**:
- Default host: `http://localhost:8787` (local development)
- Override per command with `--endpoint <URL>`
- Set `LIFESTUFF_API_ENDPOINT` for a persistent override (takes effect when `--endpoint` is omitted)
- All monetary values in dollars
- Dates in DD/MM/YYYY format

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
