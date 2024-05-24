use std::env;

/**
 * Arguments of Application
 */
pub fn arguments_app() {
    let args: Vec<String> = env::args().collect();
    let mut arguments: Vec<&str> = Vec::new();
    let num_args = args.len();
    if num_args == 0 {
        println!("No Application Args");
        std::process::exit(0);
    }
    let mut args_iter = args.iter();
    while let Some(arg) = args_iter.next() {
        if arg == "--accelerator" {
            if let Some(value) = args_iter.next() {
                std::env::set_var("ACCELERATOR_PORT", value.clone());
            }
        }
    }
    while let Some(arg) = args_iter.next() {
        arguments.push(arg.as_str())
    }
    std::env::set_var("MTA_ARGUMENTS", arguments.join(" "));
    let acc_port = env::var("ACCELERATOR_PORT").unwrap_or("".to_string());
    if !acc_port.is_empty() {
        println!("Preparing Accelerator initiation");
    } else {
        println!("accelerator has no port, starting MTA directly");
        std::process::exit(0);
    }
}