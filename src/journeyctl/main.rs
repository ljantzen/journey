use journey::app::App;
use journey::journeyctl::JourneyCtlCli;
use clap::Parser;

fn main() {
    let cli = JourneyCtlCli::parse();
    
    // Handle version flag
    if cli.version {
        println!("journeyctl {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    
    // Check if this is an init command - if so, use special initialization
    let app_result = if matches!(cli.command, Some(journey::journeyctl::Commands::Init { .. })) {
        App::new_for_init()
    } else {
        App::new()
    };
    
    match app_result {
        Ok(mut app) => {
            if let Some(command) = cli.command {
                if let Err(e) = app.run_journeyctl_command(command) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            } else {
                eprintln!("Error: 'journeyctl' requires a subcommand but one was not provided");
                eprintln!("Use 'journeyctl --help' for more information.");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize application: {}", e);
            std::process::exit(1);
        }
    }
}
