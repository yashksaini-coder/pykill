// TODO: TO create a utils for the project

pub fn get_project_path() -> Result<(), Box<dyn Error>> {

    let project_path = Path::new(project_path);
    let project_path = project_path.canonicalize()?;
}
