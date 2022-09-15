use cucumber::{FailureWriter, WorldInit};
use tokio;

use poetry::{error::PoetryError, init::Options};

mod background;
mod setup;
mod tests;

#[derive(Debug, cucumber::WorldInit, enum_as_inner::EnumAsInner)]
pub enum InitWorld {
    Initial,
    Prepared {
        options: Options,
    },
    Completed {
        options: Options,
        result: Result<(), PoetryError>,
    },
}

#[async_trait::async_trait(?Send)]
impl cucumber::World for InitWorld {
    type Error = std::convert::Infallible;

    async fn new() -> Result<Self, Self::Error> {
        return Ok(Self::Initial);
    }
}

impl Default for InitWorld {
    fn default() -> Self {
        return Self::Initial;
    }
}

#[tokio::main]
async fn main() {
    let summary = InitWorld::cucumber()
        .max_concurrent_scenarios(Some(num_cpus::get()))
        .run("./features/init")
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

#[cucumber::when("Project creation is attempted")]
async fn project_creation_is_attempted(world: &mut InitWorld) {
    let options = world.as_prepared_mut().expect("World State not set!");
    *world = InitWorld::Completed {
        options: options.clone(),
        result: poetry::init::climain(options.clone()).await,
    };
}
