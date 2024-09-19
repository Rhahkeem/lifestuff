use clap::{Args, ValueEnum};

#[derive(Debug, Args, Clone)]
pub struct Add {
    #[clap(help = "Date to add time period to", long)]
    pub date: Option<String>,
    pub val: i32,
    #[clap(help = "Time period to add to date", required = true)]
    pub period: TimePeriod,
}

//noinspection SpellCheckingInspection
#[derive(Debug, ValueEnum, Clone)]
pub enum TimePeriod {
    #[clap( aliases = ["y","yr","yrs"])]
    Years,
    #[clap( aliases = ["m", "mon"])]
    Months,
    #[clap( aliases = ["w","wk","wks"])]
    Weeks,
    #[clap( aliases = ["d"])]
    Days,
    #[clap( aliases = ["h","hr","hrs"])]
    Hours,
    #[clap( aliases = ["min","mins"])]
    Minutes,
    #[clap( aliases = ["s","secs"])]
    Seconds,
}
