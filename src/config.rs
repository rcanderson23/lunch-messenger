use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, env = "PUSHOVER_TOKEN")]
    pub pushover_token: String,

    #[arg(long, env = "PUSHOVER_USER_KEY")]
    pub pushover_user_key: String,
}
