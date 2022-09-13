use std::{fs, panic};

use anyhow::Context;
use cucumber::{given, then, when, FailureWriter, WorldInit};
use num_cpus;

use worlds::InitWorld;

mod common;
mod worlds;

#[tokio::main]
async fn main() {
    let summary = InitWorld::cucumber()
        .max_concurrent_scenarios(Some(num_cpus::get()))
        .run("./features/init.feature")
        .await;

    if summary.execution_has_failed() {
        let failed_steps = summary.failed_steps();
        let parsing_errors = summary.parsing_errors();
        panic!(
            "{} step{} failed, {} parsing error{}",
            failed_steps,
            (failed_steps != 1).then(|| "s").unwrap_or_default(),
            parsing_errors,
            (parsing_errors != 1).then(|| "s").unwrap_or_default(),
        );
    }
}

#[given("Bare project creation")]
async fn bare_project_creation(world: &mut InitWorld) {
    if !world.is_initial() {
        panic!("Using background function in scenario")
    }

    *world = {
        let cwd = tempfile::Builder::new()
            .tempdir()
            .context("Failed to create temporary directory");

        let cwd = match cwd {
            Ok(t) => t,
            Err(e) => panic!("Error occurred during creation of temporary directory: {e}"),
        };

        let author = String::from("FirstName LastName <first.last@cucumber-domain.io>");
        let name = String::from("cucumber-project");
        let description = String::from(
            "Awesome description from excellent project with real dependency management",
        );

        let mut path = cwd.path().to_owned();
        path.push(&name);

        InitWorld::Prepared {
            options: poetry::init::Options {
                path,
                name,
                description,
                author,
                license: None,
            },
        }
    };
}

#[when("Project creation is attempted")]
async fn project_creation_is_attempted(world: &mut InitWorld) {
    let options = world.as_prepared_mut().expect("World State not set!");
    *world = InitWorld::Completed {
        options: options.clone(),
        result: poetry::init::climain(options.clone()).await,
    };
}

#[given("Project directory exists in working directory")]
fn project_directory_exists(world: &mut InitWorld) {
    let options = world
        .as_prepared_mut()
        .expect("World State not set!")
        .clone();

    if let Err(e) = fs::create_dir_all(&options.path) {
        panic!("Error occurred while creating project directory: {e}")
    }
}

#[given("Project directory is not empty")]
fn project_directory_is_not_empty(world: &mut InitWorld) {
    let options = world
        .as_prepared_mut()
        .expect("World State not set!")
        .clone();

    let mut stray = options.path.clone();
    stray.push("stray");

    if let Err(e) = fs::create_dir_all(stray) {
        panic!("Failed to create temp folder in project folder: {e}")
    };
}

#[then("Project directory contains pyproject.toml")]
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

#[then(regex = r"^Creation of project directory (succeeds|fails)$")]
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
