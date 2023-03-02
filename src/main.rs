use std::process;

use msmu::auth::config::MsOAuthConfig;

fn main() {
    //extract the --env= or -e argument
    let args: Vec<String> = std::env::args().collect();
    let env_arg = args
        .iter()
        .find(|arg| {
            arg.starts_with("--env=")
                || arg.starts_with("-e=")
                || arg.starts_with("--env")
                || arg.starts_with("-e")
        })
        .unwrap_or_else(|| {
            eprintln!("Please provide the path to the config file");
            process::exit(1);
        });

    println!("env_arg: {}", env_arg);

    //remove the --env= or -e= from the argument
    let env_arg = env_arg
        .replace("--env=", "")
        .replace("-e=", "")
        .replace("--env", "")
        .replace("-e", "");

    let config = MsOAuthConfig::from_file(&env_arg).unwrap_or_else(|e| {
        eprintln!("Error reading config file: {}", e);
        process::exit(1);
    });

    if let Err(e) = msmu::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
