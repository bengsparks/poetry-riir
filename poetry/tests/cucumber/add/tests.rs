use poetry::document;

#[cucumber::then("Adding dependencies succeeds")]
fn adding_dependencies_succeeds(world: &mut crate::AddWorld) {
    let (_, _, result) = world
        .as_completed()
        .expect("Expected Completed world state!");

    if let Err(e) = result {
        panic!("Failed to add dependencies to project: {e}")
    }
}

#[cucumber::then(expr = "{word} is named in pyproject.toml")]
fn dependency_is_in_pyproject(world: &mut crate::AddWorld, dependency: String) {
    let (setup, _, _) = world
        .as_completed()
        .expect("Expected Completed world state!");

    let pyproject =
        document::load_pyproject(&setup.location).expect("Failed to load pyproject.toml");

    match pyproject.tool.poetry.dependency {
        Some(ds) => {
            if !ds.contains_key(&dependency) {
                panic!("{dependency} could not be found in pyproject.toml")
            }
        }

        None => panic!("There is no dependency section in pyproject.toml"),
    }
}
