use super::make_path_from_name;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Deserialize)]
pub(super) struct CargoWorkspace {
    members: Vec<String>
}

impl CargoWorkspace {
    pub(super) fn get_members(&self, root: &PathBuf, members: &mut HashMap<String, PathBuf>) {
        members.extend(self.members.iter().filter_map(|name| make_path_from_name(root, name)))
    }
}

#[derive(Deserialize)]
pub(super) struct CargoFile {
    workspace: Option<CargoWorkspace>
}

impl CargoFile {
    pub(super) fn get_members(&self, root: &PathBuf, members: &mut HashMap<String, PathBuf>) {
        self.workspace.map(|w| w.get_members(root, members));
    }
}