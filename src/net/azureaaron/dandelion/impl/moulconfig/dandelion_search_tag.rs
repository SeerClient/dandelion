//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionSearchTag.

#[allow(unused_imports)]
use crate::prelude::*;
use ::moulconfig::io::github::notenoughupdates::moulconfig::annotations::search_tag::SearchTag;

/// java: @SuppressWarnings("all") public class DandelionSearchTag implements SearchTag
#[derive(Clone)]
pub struct DandelionSearchTag {
    /// java: private final String tag;
    pub tag: String,
}

impl DandelionSearchTag {
    /// java: protected DandelionSearchTag(String tag)
    pub fn new(tag: String) -> Self {
        Self { tag }
    }

    /// java: @Override public Class<? extends Annotation> annotationType() - `SearchTag.class`
    pub fn annotation_type(&self) -> &'static str {
        "io.github.notenoughupdates.moulconfig.annotations.SearchTag"
    }
}

impl SearchTag for DandelionSearchTag {
    /// java: @Override public String value()
    fn value(&self) -> String {
        self.tag.clone()
    }

    /// java: @Override public String toString()
    fn to_string_java(&self) -> String {
        self.tag.clone()
    }
}

impl std::fmt::Display for DandelionSearchTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tag)
    }
}
