use evm_rust::cli::Cli;

fn main() {
    // Initialize logging
    env_logger::init();
    
    // Parse CLI arguments and run
    if let Err(e) = Cli::parse_args().run() {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}