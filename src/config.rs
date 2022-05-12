use serde::Deserialize;
use simple_error::bail;
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub dsls: Option<Vec<Dsls>>,
    pub global: Option<GlobalConfiguration>,
    pub flexlm: Option<Vec<FlexLM>>,
    pub licman20: Option<Vec<Licman20>>,
    pub lmx: Option<Vec<Lmx>>,
    pub rlm: Option<Vec<Rlm>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GlobalConfiguration {
    pub dslicsrv: Option<String>,
    pub licman20_appl: Option<String>,
    pub lmutil: Option<String>,
    pub lmxendutil: Option<String>,
    pub rlmutil: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Dsls {
    pub name: String,
    pub license: String,
    pub excluded_features: Option<Vec<String>>,
    pub export_user: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FlexLM {
    pub name: String,
    pub license: String,
    pub excluded_features: Option<Vec<String>>,
    pub export_user: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Rlm {
    pub name: String,
    pub license: String,
    pub excluded_features: Option<Vec<String>>,
    pub export_user: Option<bool>,
    pub isv: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Lmx {
    pub name: String,
    pub license: String,
    pub excluded_features: Option<Vec<String>>,
    pub export_user: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Licman20 {
    pub name: String,
    pub excluded_features: Option<Vec<String>>,
    pub export_user: Option<bool>,
}

pub fn parse_config_file(f: &str) -> Result<Configuration, Box<dyn Error>> {
    let unparsed = fs::read_to_string(f)?;
    let config: Configuration = serde_yaml::from_str(unparsed.as_str())?;

    validate_configuration(&config)?;

    Ok(config)
}

fn validate_configuration(cfg: &Configuration) -> Result<(), Box<dyn Error>> {
    if let Some(flexlm) = &cfg.flexlm {
        for flex in flexlm {
            if flex.name.is_empty() {
                bail!("Empty name for FlexLM license");
            }

            if flex.license.is_empty() {
                bail!(
                    "Missing license information for FlexLM license {}",
                    flex.name
                );
            }
        }
    }

    if let Some(rlm) = &cfg.rlm {
        for _rlm in rlm {
            if _rlm.name.is_empty() {
                bail!("Empty name for RLM license");
            }

            if _rlm.license.is_empty() {
                bail!("Missing license information for RLM license {}", _rlm.name);
            }
            if _rlm.isv.is_empty() {
                bail!("Missing ISV for RLM license {}", _rlm.name);
            }
        }
    }

    if let Some(lmx) = &cfg.lmx {
        for _lmx in lmx {
            if _lmx.name.is_empty() {
                bail!("Empty name for LM-X license");
            }

            if _lmx.license.is_empty() {
                bail!("Missing license information for LM-X license {}", _lmx.name);
            }

            for lsrv in _lmx.license.split(':') {
                if lsrv.contains('@') && lsrv.split('@').count() != 2 {
                    bail!("Invalid license for LM-X license {}", _lmx.name);
                }
            }

            if _lmx.license.contains(':') {
                let srvcnt: Vec<&str> = _lmx.license.split(':').collect();
                if srvcnt.len() != 3 {
                    bail!("Only three servers are allowed for LM-X HAL servers instead of {} for license {}", srvcnt.len(), _lmx.name);
                }
            }
        }
    }

    if let Some(dsls) = &cfg.dsls {
        for _dsls in dsls {
            if _dsls.name.is_empty() {
                bail!("Empty name for DSLS license");
            }

            if _dsls.license.is_empty() {
                bail!(
                    "Missing license information for DSLS license {}",
                    _dsls.name
                );
            }

            for lsrv in _dsls.license.split(':') {
                if !lsrv.contains('@') {
                    bail!("Invalid license for DSLS license {}", _dsls.name);
                }
            }

            if _dsls.license.contains(':') {
                let srvcnt: Vec<&str> = _dsls.license.split(':').collect();
                if srvcnt.len() != 3 {
                    bail!("Only three servers are allowed for redundant DSLS servers instead of {} for license {}", srvcnt.len(), _dsls.name);
                }
            }
        }
    }

    Ok(())
}
