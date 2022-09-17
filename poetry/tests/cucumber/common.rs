use derive_builder::Builder;

pub enum Setup {
    Location(tempfile::TempDir),
    Name(String),
    Description(String),
    Author { name: String, email: String },
    License(String),
}

pub trait ProjectSetupWorld {
    fn mut_setup_builder(&mut self) -> &mut ProjectSetupBuilder;

    fn setup(&mut self, setup: Setup) {
        let builder = self.mut_setup_builder();

        match setup {
            Setup::Location(td) => builder.location(td.path().clone().to_path_buf()),
            Setup::Name(name) => builder.name(name),
            Setup::Description(desc) => builder.description(desc),
            Setup::Author { name, email } => builder.author(format!("{name} <{email}>")),
            Setup::License(lic) => builder.license(Some(lic)),
        };
    }
}

#[derive(Builder, Clone, Debug)]
#[builder(derive(Debug))]
pub struct ProjectSetup {
    pub location: std::path::PathBuf,

    #[builder(default = r#"String::from("cucumber-project")"#)]
    pub name: String,

    #[builder(default = r#"String::from("Awesome description from excellent project with real dependency management")"#)]
    pub description: String,

    #[builder(default = r#"String::from("FirstName LastName <first.last@cucumber-domain.io>")"#)]
    pub author: String,

    #[builder(default)]
    pub license: Option<String>,
}

impl Into<poetry::init::Options> for ProjectSetup {
    fn into(self) -> poetry::init::Options {
        return poetry::init::Options {
            path: self.location,
            name: self.name,
            description: self.description,
            author: self.author,
            license: self.license.map(|l| match l.try_into() {
                Ok(l) => l,
                Err(e) => panic!("{e}"),
            }),
        };
    }
}

fn project_setup(world: &mut dyn ProjectSetupWorld, setup: Setup) {
    world.setup(setup);
}
