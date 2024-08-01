use markov_junior::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut maybe_seed = None;
    let mut maybe_output_file = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--seed" => {
                if i + 1 < args.len() {
                    maybe_seed = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    maybe_output_file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    // 4048509256541855766
    let xml = r#"
    <sequence value="B" width="175" height="175">
      <one in="B" out="W" steps="1"/>
      <one in="B" out="R" steps="1"/>
      <one>
        <rule in="RB" out="RR"/>
        <rule in="WB" out="WW"/>
      </one>
      <all in="RW" out="UU"/>
      <all>
        <rule in="W" out="B"/>
        <rule in="R" out="B"/>
      </all>
      <all in="UB" out="UU" steps="1"/>
      <all in="BU/UB" out="U*/**"/>
      <all in="UB" out="*G"/>
      <one in="B" out="E" steps="13"/>
      <one>
        <rule in="EB" out="*E"/>
        <rule in="GB" out="*G"/>
      </one>
    </sequence>
    "#;

    let mut mj = parse_xml(xml, maybe_seed);

    // let guard = pprof::ProfilerGuard::new(4999).unwrap();
    mj.generate();
    // if let Ok(report) = guard.report().build() {
    //     let file = std::fs::File::create("/tmp/flamesvg").unwrap();
    //     report.flamegraph(file).unwrap();
    // }

    if let Some(output_file) = maybe_output_file {
        let _ = mj.log_grid(output_file);
    } else {
        mj.print_grid();
    }
    println!("seed: {}", mj.seed);
    println!("patterns_applied: {}", mj.patterns_applied_counter);
}
