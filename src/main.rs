use journey::app::App;
use journey::cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    
    // Handle version flag
    if cli.version {
        println!("journey {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    
    // Initialize the app normally since init is now handled by journeyctl
    let app_result = App::new();
    
    match app_result {
        Ok(mut app) => {
            if let Err(e) = app.run(cli) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize application: {}", e);
            std::process::exit(1);
        }
    }
}
