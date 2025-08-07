use derive_builder::Builder;

#[derive(Builder, Debug)]
pub struct RepoConfig {
    name: String,
    #[builder(default)]
    age: Option<u32>,
    #[builder(default = "String::from(\"unknown\")")]
    location: String,
}