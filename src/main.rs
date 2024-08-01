use markov_junior::*;

fn main() {
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

    let mut markov = parse_xml(xml);

    // let guard = pprof::ProfilerGuard::new(4999).unwrap();
    markov.generate();
    // if let Ok(report) = guard.report().build() {
    //     let file = std::fs::File::create("/tmp/flamesvg").unwrap();
    //     report.flamegraph(file).unwrap();
    // }

    markov.print_grid();
}
