use journey::app::App;
use journey::cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    
    // Check if this is an init command - if so, use special initialization
    let app_result = if matches!(cli.command, Some(journey::cli::Commands::Init { .. })) {
        App::new_for_init()
    } else {
        App::new()
    };
    
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
