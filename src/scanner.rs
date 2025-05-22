// TODO: TO create a scanner that will scan the project for virtual environment of python using sys.prefix

pub fn scan_project(project_path: &str) -> Result<(), Box<dyn Error>> {

    let project_path = Path::new(project_path);
    let project_path = project_path.canonicalize()?;

    let project_path = project_path.to_str().unwrap();

    let project_path = project_path.to_string();

}
