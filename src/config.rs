use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, env = "PUSHOVER_TOKEN")]
    /// The Pushover application token.
    pub pushover_token: String,

    #[arg(long, env = "PUSHOVER_USER_KEY")]
    /// The Pushover user api key.
    pub pushover_user_key: String,

    #[arg(long, env = "DISTRICT")]
    /// The school district set in nutrislice domain prefix
    pub district: String,

    #[arg(long, env = "SCHOOL")]
    /// The school name used in the nutrislice path
    pub school: String,
}
