use clap::Args;

/// Check current mileage against projected mileage
#[derive(Debug, Args, Clone)]
pub struct Mileage {
    #[clap(short, long, help = "Current mileage of the vehicle")]
    pub mileage: u32,
}
