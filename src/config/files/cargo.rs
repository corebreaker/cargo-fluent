use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct CargoPackage {
    name: String
}

#[derive(Deserialize)]
pub(super) struct CargoFile {
    package: Option<CargoPackage>,
}

impl CargoFile {
    pub(super) fn name(&self) -> Option<String> {
        self.package.as_ref().map(|package| package.name.replace("-", "_"))
    }
}