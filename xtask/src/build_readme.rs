use std::{
    fs,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use cargo_toml::Manifest;
use xshell::Shell;

use crate::{template::TemplateDataForReadme, AppResult, MANUAL_NAME};

static RESOUCE_FOLDER: LazyLock<PathBuf> = LazyLock::new(|| {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("resources")
});

static TEMPLATE: LazyLock<TemplateDataForReadme> = LazyLock::new(|| {
    let path = RESOUCE_FOLDER.join("TEMPLATE_DATA.toml");
    let content = std::fs::read_to_string(path.as_path()).unwrap();
    toml::from_str(&content).unwrap()
});

pub fn build(shell: Shell, project_path: &Path) -> AppResult {
    let template_data = &TEMPLATE;
    let readme_path = project_path.join("README.md");
    let about_main = shell.read_file(RESOUCE_FOLDER.join(template_data.about()))?;
    let manual_content = shell.read_file(project_path.join(MANUAL_NAME))?;
    let manifest_content = shell.read_file(project_path.join("Cargo.toml"))?;
    let cargo_toml_file = Manifest::from_str(&manifest_content)?;
    let rust_version = cargo_toml_file.package().rust_version().unwrap();
    let application_version = cargo_toml_file.package().version();

    let readme_content = shell
        .read_file(RESOUCE_FOLDER.join(template_data.readme_template()))?
        .replace("{{ABOUT}}", &about_main)
        .replace("{{MANUAL}}", &manual_content)
        .replace("{{RUST_VERSION}}", rust_version)
        .replace("{{VERSION}}", application_version);

    fs::write(&readme_path, readme_content)?;
    Ok(())
}
