use std::process::Command;

use tauri::api::process;

pub struct ProjectCreator {
    pub _type: ProjectType,
}

impl ProjectCreator {
    pub fn new(_type: ProjectType) -> Self {
        Self { _type }
    }

    pub fn create(&self, name: String) {
        let mut cmd_raw = Command::new("cargo");
        let cmd = cmd_raw.arg("new").arg(name);
        cmd.spawn().expect("Failed to create project");
    }
}

pub enum ProjectType {
    Cargo,
}
