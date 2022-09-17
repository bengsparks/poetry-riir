use log::{info, warn};

use crate::common::{ProjectSetupWorld, Setup};

#[cucumber::given("A location for the new project")]
fn bare_project(world: &mut crate::AddWorld) {
    let cwd = tempfile::Builder::new()
        .tempdir()
        .expect("Failed to create temporary directory");

    world.setup(Setup::Location(cwd));
}

#[cucumber::given(expr = r"By the name of {string}")]
fn setup_with_name(world: &mut crate::AddWorld, name: String) {
    world.setup(Setup::Name(name))
}

#[cucumber::given(expr = r"Described by {string}")]
fn setup_with_desc(world: &mut crate::AddWorld, desc: String) {
    world.setup(Setup::Description(desc))
}

#[cucumber::given(expr = r"Written by {string}, email is {string}")]
fn setup_with_author(world: &mut crate::AddWorld, name: String, email: String) {
    world.setup(Setup::Author { name, email })
}

#[cucumber::given(expr = r"With the {string} license")]
fn setup_with_license(world: &mut crate::AddWorld, license: String) {
    world.setup(Setup::License(license))
}

#[cucumber::given("The project exists")]
async fn project_creation_is_attempted(world: &mut crate::AddWorld) {
    let setup_builder = world
        .as_initial_mut()
        .expect("Initial world state is expected");
    let setup = match setup_builder.build() {
        Ok(ps) => ps,
        Err(e) => panic!("Project creation failed with: {e}"),
    };

    let setup1 = setup.clone();

    let options = poetry::init::Options {
        path: setup.location,
        name: setup.name,
        description: setup.description,
        author: setup.author,
        license: setup.license.map(|l| match l.try_into() {
            Ok(l) => l,
            Err(e) => panic!("{e}"),
        }),
    };

    if let Err(e) = poetry::init::climain(options.clone()).await {
        panic!("Error occurred during climain invocation: {e}")
    }

    *world = crate::AddWorld::Prepared {
        working_dir: options.path.clone(),
        setup: setup1,
        add_builder: Default::default(),
    };
}
