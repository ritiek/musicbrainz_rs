extern crate musicbrainz_rs;
use musicbrainz_rs::model::place;
use musicbrainz_rs::model::place::*;
use musicbrainz_rs::Fetch;
use std::{thread, time};

// TODO : waiting for https://github.com/metabrainz/musicbrainz-server/pull/1223 to be released
// #[test]
fn should_get_place_aliases() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let blue_note = Place::fetch()
        .id("327c29c6-da63-4dc9-a117-1917ee691ce4")
        .include(place::Include::Aliases)
        .execute()
        .unwrap();

    assert!(blue_note.aliases.is_some());
}

// TODO : waiting for https://github.com/metabrainz/musicbrainz-server/pull/1223 to be released
// #[test]
fn should_get_place_tags() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let olympia = Place::fetch()
        .id("36678fc4-2fee-46be-b084-4c4e2314ce71")
        .include(place::Include::Tags)
        .execute()
        .unwrap();

    assert!(olympia.tags.is_some());
}
// TODO : waiting for https://github.com/metabrainz/musicbrainz-server/pull/1223 to be released
// #[test]
fn should_get_place_genres() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let olympia = Place::fetch()
        .id("36678fc4-2fee-46be-b084-4c4e2314ce71")
        .include(place::Include::Genres)
        .execute()
        .unwrap();

    assert!(olympia.genres.is_some());
}

#[test]
fn should_get_place_annotation() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let osaka_kosei_nenkin_kaikan = Place::fetch()
        .id("751f998a-60ca-4d48-954f-b101d59ad89a")
        .include(place::Include::Annotation)
        .execute()
        .unwrap();

    assert!(osaka_kosei_nenkin_kaikan.annotation.is_some());
}
