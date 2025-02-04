use crate::model::artist::Artist;
use crate::model::event::Event;
use crate::model::label::Label;
use crate::model::place::Place;
use crate::model::recording::Recording;
use crate::model::release::Release;
use crate::model::release_group::ReleaseGroup;
use crate::model::work::Work;

/// Browse query result are wrapped in this generic struct and paired with a custom
/// Deserialize implementation to avoid reimplementing a custom deserializer for every entity.
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

impl Browsable for Recording {
    const COUNT_FIELD: &'static str = "recording-count";
    const OFFSET_FIELD: &'static str = "recording-offset";
    const ENTITIES_FIELD: &'static str = "recordings";
}

impl Browsable for Release {
    const COUNT_FIELD: &'static str = "release-count";
    const OFFSET_FIELD: &'static str = "release-offset";
    const ENTITIES_FIELD: &'static str = "releases";
}

impl Browsable for ReleaseGroup {
    const COUNT_FIELD: &'static str = "release-group-count";
    const OFFSET_FIELD: &'static str = "release-group-offset";
    const ENTITIES_FIELD: &'static str = "release-groups";
}

impl Browsable for Work {
    const COUNT_FIELD: &'static str = "work-count";
    const OFFSET_FIELD: &'static str = "work-offset";
    const ENTITIES_FIELD: &'static str = "works";
}
