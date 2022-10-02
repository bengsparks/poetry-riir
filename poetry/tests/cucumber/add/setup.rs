use crate::AddWorld;

#[cucumber::given(expr = "The dependency {word} is to be added to the project")]
async fn add_dependency_to_project(world: &mut AddWorld, dependency: String) {
    let (_, _, add_builder) = world
        .as_prepared_mut()
        .expect("Prepared world state is expected");

    let mut dependencies = match &add_builder.deps {
        Some(d) => d.clone(),
        None => vec![],
    };
    dependencies.push(dependency);

    add_builder.deps(dependencies);
}


#[cucumber::given(expr = "The Git dependency {string} is to be added to the project")]
async fn add_git_dependency_to_project(world: &mut AddWorld, url: String) {
    let (_, _, add_builder) = world
        .as_prepared_mut()
        .expect("Prepared world state is expected");

    let mut dependencies = match &add_builder.deps {
        Some(d) => d.clone(),
        None => vec![],
    };
    dependencies.push(url);

    add_builder.deps(dependencies);
}


#[cucumber::given(expr = "The versioned dependency {string}@{string} is to be added to the project")]
async fn add_versioned_dependency_to_project(world: &mut AddWorld, name: String, version: String) {
    let (_, _, add_builder) = world
        .as_prepared_mut()
        .expect("Prepared world state is expected");

    let mut dependencies = match &add_builder.deps {
        Some(d) => d.clone(),
        None => vec![],
    };
    dependencies.push(format!("{name}@{version}"));
    add_builder.deps(dependencies);
}