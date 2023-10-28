use structmap::{FromMap, ToMap};
use structmap_derive::{FromMap, ToMap};

#[derive(Default, Debug, Clone, PartialEq, ToMap, FromMap)]
pub struct Settings {
    pub silhouette: bool,
    pub text_entry: bool,
}
