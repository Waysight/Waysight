use std::env::args;

fn print_usage() {
    let usage_str = "Waysight, the insightful wayland compositor

Usage:
    waysight [ [-a] [--arg] [-k=<value>] [--key=<value>] [-ab -c=<value>] ]

Options:
    -h        --help            Outputs the usage of the waysight command

    -c=value  --config=value    Sets the config file that waysight will be reading from.
                                Defalts to $XDG_CONFIG_HOME/waysight/waysight.toml

    -b=value  --backend=value   Sets the type of backend for waysight to run.
                                Available values are \"drm\", \"winit\", \"x11\"
                                Will automatically choose backend if option isn't set";
    println!("{}", usage_str);
}

// Parses command-line arguments made by the use
fn parse_args(args: Vec<String>) {
    for arg in &args {
        // Splits key-value pair args. The Some() arm represents key-value pair args. The None arm
        // represents args without a value attached to them
        match arg.split_once("=") {
            Some((key, value)) => match (key, value) {
                _ => {
                    print_usage();
                    return;
                }
            },
            None => {
                if arg.starts_with("--") {
                    match arg.as_str() {
                        "--help" => print_usage(),
                        _ => print_usage(),
                    }
                } else if arg.starts_with('-') {
                    arg.chars().for_each(|flag| match flag {
                        '-' => {}
                        'h' => print_usage(),
                        _ => {
                            println!("Unknown flag");
                            print_usage();
                        }
                    })
                }
            }
        }
    }
}

fn main() {
    let mut args: Vec<String> = args().collect();
    args.remove(0);
    // Saves speed by not parsing a 0 length argument vec
    if args.len() != 0 {
        parse_args(args);
    }
}
