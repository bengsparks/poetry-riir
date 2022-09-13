use poetry::{init::Options, error::PoetryError};

#[derive(Debug, cucumber::WorldInit, enum_as_inner::EnumAsInner)]
pub enum InitWorld {
    Initial,
    Prepared { options: Options },
    Completed { options: Options, result: Result<(), PoetryError> },
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