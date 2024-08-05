#[derive(Debug)]
pub enum ModelKind {
    River,
    FireNoise,
    Test,
}

impl std::str::FromStr for ModelKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "River" => Ok(ModelKind::River),
            "FireNoise" => Ok(ModelKind::FireNoise),
            "Test" => Ok(ModelKind::Test),
            _ => Err(()),
        }
    }
}

pub fn model_xml(model: ModelKind, size: usize) -> String {
    match model {
        ModelKind::River => river_xml(size),
        ModelKind::FireNoise => fire_noise_xml(size),
        ModelKind::Test => test_xml(size),
    }
}

// River https://github.com/mxgmn/MarkovJunior/blob/main/models/River.xml
fn river_xml(size: usize) -> String {
    format!(
        r#"
        <sequence fill="B" width="{size}" height="{size}">
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
        "#
    )
}

// FireNoise https://github.com/mxgmn/MarkovJunior/blob/main/models/FireNoise.xml
fn fire_noise_xml(size: usize) -> String {
    format!(
        r#"
        <sequence fill="B" width="{size}" height="{size}">
          <prl steps="75">
            <rule in="OG" out="*O"/>
            <rule in="O*/*G" out="**/*O"/>
            <rule in="B" out="G" p="0.01"/>
            <rule in="O" out="B"/>
            <rule in="G" out="O" p="0.0001"/>
          </prl>
          <all in="*G*/GBG" out="***/*G*"/>
          <all>
            <rule in="*B*/BGB/*B*" out="***/*B*/***"/>
            <rule in="*BB*/BGGB/*BB*" out="****/*BB*/****"/>
          </all>
          <sequence>
            <one in="G" out="R" steps="1"/>
            <all in="RG" out="RR" steps="10"/>
            <all in="RG" out="EE"/>
            <all>
              <rule in="ER" out="*E"/>
              <rule in="EG" out="*E"/>
            </all>
          </sequence>
          <sequence>
            <one in="B" out="K" steps="1"/>
            <all in="KB" out="*K" steps="10"/>
            <all in="KB" out="GG"/>
            <all>
              <rule in="GB" out="*G"/>
              <rule in="GK" out="*G"/>
            </all>
          </sequence>
          <prl>
            <rule in="K" out="E"/>
            <rule in="G" out="B"/>
          </prl>
        </sequence>
        "#
    )
}

fn test_xml(size: usize) -> String {
    format!(
        r#"
        <sequence fill="B" width="{size}" height="{size}">
          <prl steps="75">
            <rule in="OG" out="*O"/>
            <rule in="O*/*G" out="**/*O"/>
            <rule in="B" out="G" p="0.01"/>
            <rule in="O" out="B"/>
            <rule in="G" out="O" p="0.0001"/>
          </prl>
          <all in="*G*/GBG" out="***/*G*"/>
          <all>
            <rule in="*B*/BGB/*B*" out="***/*B*/***"/>
            <rule in="*BB*/BGGB/*BB*" out="****/*BB*/****"/>
          </all>
        </sequence>
        "#
    )
}
