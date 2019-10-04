use crate::model::artist::Artist;
use crate::model::event::Event;
use crate::model::label::Label;
use crate::model::place::Place;

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct BrowseResult<T> {
    pub count: i32,
    pub offset: i32,
    pub entities: Vec<T>,
}

pub trait Browsable {
    const COUNT_FIELD: &'static str;
    const OFFSET_FIELD: &'static str;
    const ENTITIES_FIELD: &'static str;
}

impl Browsable for Artist {
    const COUNT_FIELD: &'static str = "artist-count";
    const OFFSET_FIELD: &'static str = "artist-offset";
    const ENTITIES_FIELD: &'static str = "artists";
}

impl Browsable for Event {
    const COUNT_FIELD: &'static str = "event-count";
    const OFFSET_FIELD: &'static str = "event-offset";
    const ENTITIES_FIELD: &'static str = "events";
}

impl Browsable for Label {
    const COUNT_FIELD: &'static str = "label-count";
    const OFFSET_FIELD: &'static str = "label-offset";
    const ENTITIES_FIELD: &'static str = "labels";
}

impl Browsable for Place {
    const COUNT_FIELD: &'static str = "place-count";
    const OFFSET_FIELD: &'static str = "place-offset";
    const ENTITIES_FIELD: &'static str = "places";
}
