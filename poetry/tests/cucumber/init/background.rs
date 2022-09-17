use std::convert::TryInto;

use anyhow::Context;

use crate::InitWorld;

#[cucumber::given("Bare project settings")]
async fn bare_project_settings(world: &mut InitWorld) {
    if !world.is_initial() {
        panic!("Using background function in scenario")
    }

    *world = {
        let cwd = tempfile::Builder::new()
            .tempdir()
            .expect("Failed to create temporary directory");

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

#[cucumber::given(expr = r"Licensed project with {word}")]
async fn project_settings_with_license(world: &mut InitWorld, spec_license: String) {
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

        let license: Option<_> = match spec_license.clone().try_into() {
            Ok(l) => Some(l),
            Err(e) => panic!("Failed to create license from {spec_license}: {e}"),
        };
        InitWorld::Prepared {
            options: poetry::init::Options {
                path,
                name,
                description,
                author,
                license,
            },
        }
    };
}
