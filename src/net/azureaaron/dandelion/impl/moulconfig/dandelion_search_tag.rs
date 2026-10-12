//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionSearchTag by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @SuppressWarnings("all") public class DandelionSearchTag implements SearchTag
#[derive(Clone)]
pub struct DandelionSearchTag {
    /// java: private final String tag;
    pub tag: String,
}

impl DandelionSearchTag {
    /// java: protected DandelionSearchTag(String tag)
    pub fn new(mut tag: String) -> Self {
        let __init_tag = tag.clone();
        let mut this = Self {
            tag: __init_tag,
        };
        this
    }

    /// java: @Override public Class<? extends Annotation> annotationType()
    pub fn annotation_type(&self) -> crate::unresolved::Class {
        // TODO(fabric2seer): class literal
        //   java: return SearchTag.class;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return self.tag.clone();
    }

    /// java: @Override public String value()
    pub fn value(&self) -> String {
        return self.tag.clone();
    }

}

impl std::fmt::Display for DandelionSearchTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", DandelionSearchTag::to_string(self))
    }
}
