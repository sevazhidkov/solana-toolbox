use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "solana_toolbox_cli")]
enum Cli {
    account: CliAccount,
}

enum CliAccount {
    address: String,
}

fn main() {
    let cli = Cli::parse();

    eprintln("cli: {:#?}", cli);
}
