use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "scanner")]
pub struct Cli {
    pub target: String,

    #[arg(short, long, default_value = "1-1024")]
    pub ports: String,

    #[arg(short, long, default_value_t = 100)]
    pub concurrency: usize,

    #[arg(short = 't', long, default_value = "tcp")]
    pub scan_type: String,
}
