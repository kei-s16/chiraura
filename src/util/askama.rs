use askama::Template;

use super::super::types::Profile;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    profile: &'a Profile,
}

pub fn generate_page(_profile: Profile) -> std::string::String {
    let index = IndexTemplate { profile: &_profile };
    index.render().unwrap()
}
