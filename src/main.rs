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
    from: FromUnits,
    value: f64,
    // #[clap(visible_aliases=["a","acres"], long, action=clap::ArgAction::SetTrue, group="area")]
    // to_acres: Option<bool>,
    // #[clap(long, visible_aliases=["squarefeet","sqft"], action=clap::ArgAction::SetTrue, group="area")]
    // to_sqft: Option<bool>,
    // #[clap(visible_aliases=["metres","m"], long, action=clap::ArgAction::SetTrue, group="area")]
    // to_metres: Option<bool>,
    // #[clap( visible_aliases=["inches","i"],long, action=clap::ArgAction::SetTrue, group="area")]
    // to_inches: Option<bool>,
    // #[clap(long,visible_aliases=["mi","miles"], action=clap::ArgAction::SetTrue, group="area")]
    // to_miles: Option<bool>,
    // #[clap(long, visible_aliases=["km","kilometres"], action=clap::ArgAction::SetTrue, group="area")]
    // to_kilometres: Option<bool>,
    #[clap(long)]
    to: Vec<ToUnits>,
}

#[derive(Subcommand, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FromUnits {
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
    /// Convert From Square Kilometres
    Kilometres,
}

#[derive(Subcommand, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ToUnits {
    /// Convert To Acres
    Acres,
    /// Convert To Square Feet
    Sqft,
    /// Convert To Square Metres
    Metres,
    /// Convert To Square Inches
    Inches,
    /// Convert To Square Miles
    Miles,
    /// Convert To Square Kilometres
    Kilometres,
}

// LOOKS LIKE ->>> prog convert {options} <val> --to{options}

// prog convert from {options} <val> to {options}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Convert(blah) => {
            println!("Got blah {:?}", blah);
            // match &blah.from {
            //     Units::Acres => println!("Got Acres!!!"),
            //     Units::Inches => println!("Got Inches!!!"),
            //     Units::Kilometres => println!("Got Kilometres!!!"),
            //     Units::Metres => println!("Got Metres!!"),
            //     Units::Miles => println!("Got Miles!!"),
            //     Units::Sqft => println!("Got Squarefeet"),
            // }
        }
    }
}

// compound interest
