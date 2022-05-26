mod recognition;
mod synthesis;

use structopt::StructOpt;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(StructOpt)]
#[structopt(name = "Speech-Center", author = "Verbio Technologies S.L.", version = VERSION)]
enum Args {
    Recognition(recognition::Recognition),
    Synthesis(synthesis::Synthesis),
}

#[tokio::main]
async fn main() {
    match Args::from_args() {
        Args::Recognition(c) => recognition::process_subcommand(c).await,
        Args::Synthesis(c) => synthesis::process_subcommand(c).await,
    }
}
