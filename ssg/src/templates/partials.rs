//! Represents all templates in templates/partials

use serde::Deserialize;

#[derive(Deserialize)]

pub(crate)  struct MetaTemplate {
    pub(crate) title: String
}

#[derive(Deserialize)]
pub(crate)  struct NavbarTemplate {
    pub(crate) links: Vec<Url>
}

#[derive(Deserialize)]
pub(crate)  struct Claim {
    pub(crate) title: String,
    pub(crate)  description: String
}

#[derive(Deserialize)]
pub(crate)  struct Url {
    pub(crate) url: String,
    pub(crate) title: String
}