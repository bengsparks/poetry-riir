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