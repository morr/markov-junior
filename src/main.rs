use markov_junior::*;
use std::env;

const DEFAULT_SIZE: usize = 100;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut maybe_seed = None;
    let mut size = DEFAULT_SIZE;
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
                    size = match args[i + 1].parse::<usize>() {
                        Ok(parsed_size) => parsed_size,
                        Err(_) => {
                            eprintln!(
                                "Error: Invalid size value. Using default size {}",
                                DEFAULT_SIZE
                            );
                            DEFAULT_SIZE
                        }
                    };
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

    // River https://github.com/mxgmn/MarkovJunior/blob/main/models/River.xml
    // let xml = format!(
    //     r#"
    //     <sequence fill="B" width="{size}" height="{size}">
    //       <one in="B" out="W" steps="1"/>
    //       <one in="B" out="R" steps="1"/>
    //       <one>
    //         <rule in="RB" out="RR"/>
    //         <rule in="WB" out="WW"/>
    //       </one>
    //       <all in="RW" out="UU"/>
    //       <all>
    //         <rule in="W" out="B"/>
    //         <rule in="R" out="B"/>
    //       </all>
    //       <all in="UB" out="UU" steps="1"/>
    //       <all in="BU/UB" out="U*/**"/>
    //       <all in="UB" out="*G"/>
    //       <one in="B" out="E" steps="13"/>
    //       <one>
    //         <rule in="EB" out="*E"/>
    //         <rule in="GB" out="*G"/>
    //       </one>
    //     </sequence>
    //     "#
    // );

    // FireNoise https://github.com/mxgmn/MarkovJunior/blob/main/models/FireNoise.xml
    let xml = format!(
        r#"
        <sequence fill="B" width="{size}" height="{size}">
          <prl steps="75">
            <rule in="OG" out="*O"/>
            <rule in="O*/*G" out="**/*O"/>
            <rule in="B" out="G" p="0.01"/>
            <rule in="O" out="B"/>
            <rule in="G" out="O" p="0.0001"/>
          </prl>
        </sequence>
        "#
    );
   // let xml = format!(
   //      r#"
   //      <sequence fill="B" width="{size}" height="{size}">
   //        <prl steps="75">
   //          <rule in="OG" out="*O"/>
   //          <rule in="O*/*G" out="**/*O"/>
   //          <rule in="B" out="G" p="0.01"/>
   //          <rule in="O" out="B"/>
   //          <rule in="G" out="O" p="0.0001"/>
   //        </prl>
   //        <all in="*G*/GBG" out="***/*G*"/>
   //        <all>
   //          <rule in="*B*/BGB/*B*" out="***/*B*/***"/>
   //          <rule in="*BB*/BGGB/*BB*" out="****/*BB*/****"/>
   //        </all>
   //        <markov>
   //          <sequence>
   //            <one in="G" out="R" steps="1"/>
   //            <all in="RG" out="RR" steps="10"/>
   //            <all in="RG" out="EE"/>
   //            <all>
   //              <rule in="ER" out="*E"/>
   //              <rule in="EG" out="*E"/>
   //            </all>
   //          </sequence>
   //        </markov>
   //        <markov>
   //          <sequence>
   //            <one in="B" out="K" steps="1"/>
   //            <all in="KB" out="*K" steps="10"/>
   //            <all in="KB" out="GG"/>
   //            <all>
   //              <rule in="GB" out="*G"/>
   //              <rule in="GK" out="*G"/>
   //            </all>
   //          </sequence>
   //        </markov>
   //        <prl>
   //          <rule in="K" out="E"/>
   //          <rule in="G" out="B"/>
   //        </prl>
   //      </sequence>
   //      "#
   //  );
    let mut mj = parse_xml(&xml, maybe_seed);

    for rule_index in 0..mj.rules.len() {
        if let Some(ref output_file) = maybe_output_file {
            log(&mj, output_file, maybe_log_cmd.as_deref());
        }
        mj.generate(rule_index);
    }

    if let Some(output_file) = maybe_output_file {
        log(&mj, &output_file, maybe_log_cmd.as_deref());
    } else {
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
