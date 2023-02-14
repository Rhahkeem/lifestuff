use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[clap(
    author = "Me!",
    about = "Lifestyle library!",
    version = "0.1.0",
    long_about = "Something Something Something Daaaarksiiiide"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Specify unit conversion option
    Convert(Conversions),
}

#[derive(Debug, Args)]
struct Conversions {
    #[arg(value_enum)]
    from: Units,
}

#[derive(Subcommand, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Units {
    /// Convert From Acres
    Acres,
    /// Convert From Square Feet
    Sqft,
    /// Convert From Square Metres
    Metres,
    /// Convert From Square Inches
    Inches,
    /// Convert From Square Miles
    Miles,
    /// Convert From Square Feet
    Kilometres,
}
// LOOKS LIKE ->>> prog convert {options} <val> --to{options}

// #[derive(Debug, Args)]
// struct FromUnits {
//     #[clap(group = "FromArg")]
//     acres: f64,
//     sqft: f64,
//     metres: f64,
//     inches: f64,
//     miles: f64,
// }

// #[derive(Debug, Args)]
// struct ToUnits {
//     #[clap(group = "FromArg")]
//     acres: f64,
//     sqft: f64,
//     metres: f64,
//     inches: f64,
//     miles: f64,
// }

// prog convert from {options} <val> to {options}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Convert(blah) => {
            println!("Got blah {:?}", blah.from);
            match &blah.from {
                Units::Acres => println!("Got Acres!!!"),
                Units::Inches => println!("Got Inches!!!"),
                Units::Kilometres => println!("Got Kilometres!!!"),
                Units::Metres => println!("Got Metres!!"),
                Units::Miles => println!("Got Miles!!"),
                Units::Sqft => println!("Got Squarefeet"),
            }
        }
    }
}

// acres to sq ft
// metres to inches
// km to miles
// compound interest
