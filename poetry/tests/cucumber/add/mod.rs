use cucumber::{StatsWriter, World};
use tokio;

use poetry::{add, error::PoetryError};

mod background;
mod setup;
mod tests;

#[path = "../common.rs"]
mod common;

#[derive(Debug, cucumber::World, enum_as_inner::EnumAsInner)]
pub enum AddWorld {
    Initial {
        setup_builder: common::ProjectSetupBuilder,
    },
    Prepared {
        working_dir: std::path::PathBuf,
        setup: common::ProjectSetup,
        add_builder: add::OptionsBuilder,
    },
    Completed {
        setup: common::ProjectSetup,
        add: add::Options,
        result: Result<(), PoetryError>,
    },
}

impl Default for AddWorld {
    fn default() -> Self {
        return Self::Initial {
            setup_builder: Default::default(),
        };
    }
}

impl common::ProjectSetupWorld for AddWorld {
    fn mut_setup_builder(&mut self) -> &mut common::ProjectSetupBuilder {
        let init = self
            .as_initial_mut()
            .expect("Project Setup requires the Initial state!");
        return init;
    }
}

#[tokio::main]
async fn main() {
    let summary = AddWorld::cucumber()
        .max_concurrent_scenarios(Some(num_cpus::get()))
        .run("./features/add")
        .await;

    if summary.execution_has_failed() {
        let failed_steps = summary.failed_steps();
        let parsing_errors = summary.parsing_errors;
        panic!(
            "{} step{} failed, {} parsing error{}",
            failed_steps,
            (failed_steps != 1).then(|| "s").unwrap_or_default(),
            parsing_errors,
            (parsing_errors != 1).then(|| "s").unwrap_or_default(),
        );
    }
}

#[cucumber::when("The dependencies are added to the project")]
async fn add_dependencies_to_project(world: &mut AddWorld) {
    let (_, setup, add_builder) = world
        .as_prepared_mut()
        .expect("Prepared world state is expected");

    add_builder.working_directory(setup.location.clone());

    let add = match add_builder.build() {
        Ok(add) => add,
        Err(e) => panic!("Failed to create add::Options from builder: {e}"),
    };

    *world = AddWorld::Completed {
        setup: setup.clone(),
        add: add.clone(),
        result: poetry::add::climain(add.clone()).await,
    };
}
