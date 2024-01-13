//! Represents the index.sptl template

use sailfish::TemplateOnce;
use serde::Deserialize;

use super::partials::{MetaTemplate, NavbarTemplate, Claim};

#[derive(TemplateOnce, Deserialize)]
#[template(path = "index.stpl")]
pub(crate) struct IndexTemplate {
    meta: MetaTemplate,
    navbar: NavbarTemplate,
    claims: Vec<Claim>
}