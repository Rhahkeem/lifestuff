# Lifestuff - Project Documentation

## Overview

**Lifestuff** is a Rust-based command-line utility designed to handle various lifestyle calculations and conversions. It provides a unified interface for unit conversions, date operations, financial calculations, currency conversions, mileage tracking, and DuckDuckGo email utilities.

## Project Structure

This is a Rust workspace with two main crates:

```
lifestuff/
├── Cargo.toml                    # Workspace configuration
├── lifestuff/                    # Main binary crate
│   ├── Cargo.toml               # Binary dependencies
│   ├── build.rs                 # Build script for shell completions
│   ├── src/
│   │   ├── main.rs              # Entry point and command routing
│   │   ├── conversions.rs       # Unit conversion implementations
│   │   ├── currency.rs          # Currency conversion logic
│   │   ├── dateinfo.rs          # Date operation implementations
│   │   ├── ddg.rs               # DuckDuckGo utilities
│   │   ├── interest.rs          # Financial calculations
│   │   └── mileage.rs           # Mileage tracking
│   └── README.md                # User documentation
└── lifestuff-types/             # Shared types and CLI definitions
    ├── Cargo.toml               # Types crate dependencies
    └── src/
        ├── lib.rs               # Main CLI structure and parser
        ├── conversions.rs       # Conversion command types
        ├── currency.rs          # Currency command types
        ├── dateinfo.rs          # Date command types
        ├── ddg.rs               # DuckDuckGo command types
        ├── interest.rs          # Interest command types
        ├── mileage.rs           # Mileage command types
        └── conversions/         # Conversion submodules
            ├── area.rs          # Area conversion types
            └── distance.rs      # Distance conversion types
```

## Architecture

### Command Structure

The application uses **clap** for CLI parsing with a hierarchical command structure:

```
lifestuff [--verbose] <COMMAND>
├── convert
│   ├── area
│   └── distance
├── dates
│   ├── add
│   ├── diff
│   └── ordinal
├── interest
├── currency
├── mileage
└── ddg
    ├── generate
    └── convert
```

### Key Design Patterns

1. **Separation of Concerns**: CLI definitions are separated from business logic
   - `lifestuff-types`: Contains all CLI argument structures and parsing
   - `lifestuff`: Contains implementation logic for each command

2. **Modular Architecture**: Each feature is implemented as a separate module with its own types and logic

3. **Error Handling**: Uses `anyhow` for comprehensive error handling throughout the application

## Features

### 1. Unit Conversions (`convert`)
- **Area conversions**: Convert between different area units
- **Distance conversions**: Convert between different distance units
- Extensible design for adding more conversion types

### 2. Date Operations (`dates`)
- **Add**: Add time periods to dates
- **Diff**: Calculate differences between two dates  
- **Ordinal**: Get ordinal date information
- Uses the `time` crate for robust date handling

### 3. Interest Calculations (`interest`)
Comprehensive mortgage/loan calculations with:
- Principal amount tracking
- Interest rate calculations
- Monthly payment planning
- Annual repayment limits (percentage or fixed amount)
- End date projections
- Supports both percentage-based and fixed annual downpayment limits

### 4. Currency Conversion (`currency`)
- Convert between different currencies
- Support for multiple target currencies in a single command
- Uses external APIs for real-time exchange rates (via `reqwest`)

### 5. Mileage Tracking (`mileage`)
- Track current vehicle mileage against projected values
- Useful for lease agreements and maintenance scheduling

### 6. DuckDuckGo Email Utilities (`ddg`)
- **Generate**: Create DuckDuckGo email alias addresses
- **Convert**: Transform regular email addresses for DuckDuckGo forwarding

## Build System

### Shell Completion Generation

The project includes a sophisticated build script (`build.rs`) that automatically generates shell completions:

**When**: Completions are generated during every `cargo build`
**How**: Uses `clap_complete` to introspect the CLI structure and generate completion scripts
**Output**: Zsh completion file `_lifestuff` placed in `$OUT_DIR/completion/` or `$RUST_COMPLETION_DIR`

The completion generation process:
1. Imports the `Cli` struct from `lifestuff-types`
2. Uses `clap::CommandFactory` to create a command instance
3. Calls `clap_complete::generate_to()` to create the Zsh completion script
4. Outputs to a configurable directory

### Dependencies

**Core Dependencies:**
- `clap` (v4): Command-line argument parsing with derive macros
- `anyhow` (v1): Error handling and context
- `time` (v0.3): Date and time operations
- `reqwest`: HTTP client for API calls (currency conversion)
- `serde`/`serde_json`: JSON serialization for API responses
- `colored` (v2): Terminal output coloring
- `strum`: Enum utilities and string conversions
- `regex` (v1): Pattern matching

**Build Dependencies:**
- `clap_complete`: Shell completion generation

**Development Dependencies:**
- `dotenv`: Environment variable loading for testing
- `mockito`: HTTP mocking for tests

## Usage Patterns

### Global Options
- `--verbose` (`-v`): Enable detailed output across all commands
- `--help` (`-h`): Context-sensitive help for any command or subcommand
- `--version` (`-V`): Display version information

### Command Examples

```bash
# Unit conversions
lifestuff convert area --from "square-meters" --to "square-feet" --value 100
lifestuff convert distance --from "kilometers" --to "miles" --value 50

# Date operations
lifestuff dates add --date "2024-01-01" --period "3 months"
lifestuff dates diff --start "2024-01-01" --end "2024-12-31"

# Financial calculations
lifestuff interest --principal 250000 --interest-rate 3.5 --repayment 1200 \
  --max-repayment-pct 10 --end-date "31/12/2030"

# Currency conversion
lifestuff currency --from USD --amt 100 --to EUR GBP JPY

# Mileage tracking
lifestuff mileage --mileage 45000

# DuckDuckGo utilities
lifestuff ddg generate
lifestuff ddg convert --email "user@example.com"
```

## Development

### Building
```bash
cargo build          # Build with shell completions
cargo check          # Quick syntax check
cargo test            # Run test suite
```

### Installing
```bash
cargo install --path lifestuff/    # Install from source
```

### Testing
The project includes comprehensive tests for each module, with mocking support for external API calls.

## Technical Notes

### Error Handling Strategy
- Uses `anyhow::Result<()>` for consistent error propagation
- Errors are displayed to users in a friendly format
- Verbose mode provides additional debugging information

### External API Integration
- Currency conversion uses real-time exchange rate APIs
- HTTP requests are handled synchronously with `reqwest::blocking`
- Includes proper error handling for network failures

### Shell Integration
- Automatic Zsh completion generation enhances user experience
- Completions include all commands, subcommands, and options
- Can be installed system-wide or per-user

This project demonstrates modern Rust CLI development practices with a focus on user experience, maintainability, and extensibility.
