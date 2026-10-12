//! Ported from net.azureaaron.dandelion.Dandelion by fabric2seer. Entrypoint: client.

#[allow(unused_imports)]
use crate::prelude::*;

use mcrust::net::minecraft::resources::identifier::Identifier;


pub fn on_initialize_client() {
}

pub fn id(path: &str) -> Identifier {
    return Identifier::from_namespace_and_path("dandelion", path);
}
