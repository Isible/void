use crate::projects::{ProjectCreator, ProjectType};

#[tauri::command]
pub fn create_project(name: String) {
    let project_creator = ProjectCreator::new(ProjectType::Cargo);
    project_creator.create(name);
}
