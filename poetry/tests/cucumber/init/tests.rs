use poetry::document;

use crate::InitWorld;

#[cucumber::then("Project directory contains pyproject.toml")]
fn project_directory_contains_pyproject_toml(world: &mut InitWorld) {
    let (options, _) = world.as_completed().expect("World State not set!");
    let mut pyproject_path = options.path.clone();
    pyproject_path.push("pyproject.toml");

    if !pyproject_path.exists() {
        panic!(
            "Failed to create pyproject.toml in project folder @ {folder}",
            folder = options.path.display()
        );
    }
}

#[cucumber::then(regex = r"^Creation of project directory (succeeds|fails)$")]
fn creation_of_project_directory(world: &mut InitWorld, status: String) {
    let (options, result) = world.as_completed().expect("World State not set!");

    match (status.as_str(), result) {
        ("succeeds", Err(e)) => panic!(
            "Could not create project folder @ {folder}: {e}",
            folder = options.path.display()
        ),
        ("fails", Ok(_)) => panic!(
            "Should not have created project folder @ {folder}",
            folder = options.path.display()
        ),
        _ => {}
    }
}

#[cucumber::then(expr = "License key is set in pyproject.toml as {word}")]
fn license_key_is_set_in_pyproject(world: &mut InitWorld, license: String) {
    let (options, _) = world.as_completed().expect("World State not set!");

    let config = document::load_pyproject(&options.path).expect("Could not read from pyproject.toml");
    let value = config.tool.poetry.license.expect("License key was not set at all");

    if value != license {
        panic!("Expected license value to be {license}; got {value}")
    }
}