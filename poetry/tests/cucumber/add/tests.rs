use poetry::{
    pyproject::{self, DependencyMetadata, PyProject},
    virtualenv::Virtualenv,
};

use crate::AddWorld;

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

    let pyproject = PyProject::load(&setup.proj_location).expect("Failed to load pyproject.toml");

    match pyproject.tool.poetry.dependency {
        Some(ds) => {
            if !ds.contains_key(&dependency) {
                panic!("{dependency} could not be found in pyproject.toml")
            }
        }

        None => panic!("There is no dependency section in pyproject.toml"),
    }
}

#[cucumber::then(expr = "{word} is named in pyproject.toml with a git key with the value {string}")]
fn dependency_is_in_pyproject_with_git_key(
    world: &mut crate::AddWorld,
    dependency: String,
    url: String,
) {
    let (setup, _, _) = world
        .as_completed()
        .expect("Expected Completed world state!");

    let pyproject = PyProject::load(&setup.proj_location).expect("Failed to load pyproject.toml");

    let metadata = match pyproject.tool.poetry.dependency {
        Some(ref ds) => {
            if let Some(metadata) = ds.get(&dependency) {
                metadata
            } else {
                panic!("{dependency} could not be found in pyproject.toml")
            }
        }

        None => panic!("There is no dependency section in pyproject.toml"),
    };

    match metadata {
        DependencyMetadata::Git(pyproject::Git::Plain { git }) => {
            if git != &url {
                panic!("Expected {url} for {dependency}, got {git}");
            }
        }
        _ => panic!("Found {metadata:?} instead"),
    }
}

#[cucumber::then(regex = r"^Creation of virtual environment (succeeds|fails)$")]
fn creation_of_virtual_env(world: &mut AddWorld, status: String) {
    let (setup, _, _) = world.as_completed().expect("World State not set!");

    let venv_res = Virtualenv::from_preexisting_venv(&setup.proj_location);
    match (status.as_str(), &venv_res) {
        ("succeeds", Err(e)) => {
            panic!(
                "Expected virtualenv creation to succeed for {loc}, failed instead:\n{e}",
                loc = setup.proj_location.display()
            );
        }
        ("fails", Ok(_)) => {
            panic!(
                "Expected virtualenv creation to fail for {loc}, succeeded instead",
                loc = setup.proj_location.display()
            )
        }
        _ => {}
    }
}

#[cucumber::then(expr = "{string} is installed in the virtual environment")]
fn dependency_is_installed(world: &mut AddWorld, dependency: String) {
    let (setup, _, _) = world.as_completed().expect("World State not set!");

    let virtualenv = Virtualenv::from_preexisting_venv(&setup.proj_location).expect(&format!(
        "Unable to read virtualenv from project @ {loc}",
        loc = setup.proj_location.display()
    ));

    if !virtualenv.contains_module(&dependency) {
        panic!("Unable to find {dependency}")
    }
}
