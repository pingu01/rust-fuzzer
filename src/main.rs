use colored::Colorize;
use rust_fuzzer::fuzzer::Fuzzer;

fn print_banner() {
    let banner = r#"
    ██████╗ ██╗   ██╗███████╗████████╗    ███████╗██╗   ██╗███████╗███████╗███████╗██████╗
    ██╔══██╗██║   ██║██╔════╝╚══██╔══╝    ██╔════╝██║   ██║╚══███╔╝╚══███╔╝██╔════╝██╔══██╗
    ██████╔╝██║   ██║███████╗   ██║       █████╗  ██║   ██║  ███╔╝   ███╔╝ █████╗  ██████╔╝
    ██╔══██╗██║   ██║╚════██║   ██║       ██╔══╝  ██║   ██║ ███╔╝   ███╔╝  ██╔══╝  ██╔══██╗
    ██║  ██║╚██████╔╝███████║   ██║       ██║     ╚██████╔╝███████╗███████╗███████╗██║  ██║
    ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝       ╚═╝      ╚═════╝ ╚══════╝╚══════╝╚══════╝╚═╝  ╚═╝
    "#;
    println!("{}", banner.bright_red().bold());
    println!(
        "{}",
        "                        Web Directory Fuzzer v0.1.0".bright_cyan()
    );
    println!();
}

#[tokio::main]
async fn main() {
    print_banner();

    let args = std::env::args().collect::<Vec<String>>();
    let usage = format!("{}", "Usage: cargo run <url> <wordlist>".yellow());
    let options = format!(
        "{}",
        "Options
        -h, --help    Show this help message
        -v, --version Show version information"
            .cyan()
    );

    //usage and version parser
    if args.len() < 3 || args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        println!("{}", usage);
        println!("{}", options);
        return;
    }

    if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        println!("{}", "Rust Fuzzer version 0.1.0".green().bold());
        return;
    }

    //get url and wordlist from args
    let url = &args[1];
    let wordlist = &args[2];

    //execute fuzzer
    println!("{} {}", "URL:".bright_blue().bold(), url.white());
    println!("{} {}", "Wordlist:".bright_blue().bold(), wordlist.white());

    let fuzzer = Fuzzer::new(url, wordlist.into()).unwrap();

    fuzzer.run_default().await;
}
