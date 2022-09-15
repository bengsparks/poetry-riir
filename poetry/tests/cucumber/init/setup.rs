use crate::InitWorld;

#[cucumber::given("Project directory exists in working directory")]
fn project_directory_exists(world: &mut InitWorld) {
    let options = world
        .as_prepared_mut()
        .expect("World State not set!")
        .clone();

    if let Err(e) = std::fs::create_dir_all(&options.path) {
        panic!("Error occurred while creating project directory: {e}")
    }
}

#[cucumber::given("Project directory is not empty")]
fn project_directory_is_not_empty(world: &mut InitWorld) {
    let options = world
        .as_prepared_mut()
        .expect("World State not set!")
        .clone();

    let mut stray = options.path.clone();
    stray.push("stray");

    if let Err(e) = std::fs::create_dir_all(stray) {
        panic!("Failed to create temp folder in project folder: {e}")
    };
}

