use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateDataForReadme {
    readme_template: PathBuf,
    about: PathBuf,
}

impl TemplateDataForReadme {
    pub fn readme_template(&self) -> &PathBuf {
        &self.readme_template
    }

    pub fn about(&self) -> &PathBuf {
        &self.about
    }
}
