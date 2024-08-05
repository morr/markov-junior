use markov_junior::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut maybe_seed = None;
    let mut maybe_size = None;
    let mut maybe_model = None;
    let mut maybe_output_file = None;
    let mut maybe_log_cmd = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--seed" => {
                if i + 1 < args.len() {
                    maybe_seed = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            "--size" => {
                if i + 1 < args.len() {
                    maybe_size = args[i + 1].parse::<usize>().ok();
                    i += 1;
                }
            }
            "--model" => {
                if i + 1 < args.len() {
                    maybe_model = args[i + 1].parse::<ModelKind>().ok();
                    i += 1;
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    maybe_output_file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--log_cmd" => {
                if i + 1 < args.len() {
                    maybe_log_cmd = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    let model = maybe_model.expect("Error: --model argument is required");
    let size = maybe_size.expect("Error: --size argument is required");

    let xml = model_xml(model, size);
    let (mut mj, sequence) = parse_xml(&xml, maybe_seed);

    for rule in &sequence.rules {
        mj.generate(rule);
        if let Some(ref output_file) = maybe_output_file {
            log(&mj, output_file, maybe_log_cmd.as_deref());
        }
    }

    // if let Some(output_file) = maybe_output_file {
    //     log(&mj, &output_file, maybe_log_cmd.as_deref());
    // } else {
    //     mj.print_grid();
    // }
    if maybe_output_file.is_none() {
        mj.print_grid();
    }

    println!("seed: {}", mj.seed);
    println!("changes: {}", mj.changes);
}

fn log(mj: &MarkovJunior, output_file: &str, maybe_log_cmd: Option<&str>) {
    mj.log_grid(output_file.to_string());

    if let Some(ref log_cmd) = maybe_log_cmd {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(log_cmd)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .expect("Failed to execute shell command");
    }
}
