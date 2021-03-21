extern crate musicbrainz_rs;
use musicbrainz_rs::model::series;
use musicbrainz_rs::model::series::Series;
use musicbrainz_rs::Fetch;
use std::{thread, time};

#[test]
fn should_get_serie_tags() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let breaks_loop_n_edits = Series::fetch()
        .id("0c66e70d-5f23-4579-8fe5-6bc0007428a2")
        .include(series::Include::Tags)
        .execute()
        .unwrap();

    assert!(breaks_loop_n_edits
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "breakbeat"));
}

#[test]
fn should_get_serie_aliases() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let ultimate_breaks_and_beats = Series::fetch()
        .id("3e5979c8-5a78-4d0b-878a-0fb87853effe")
        .include(series::Include::Aliases)
        .execute()
        .unwrap();

    assert!(ultimate_breaks_and_beats.aliases.is_some());
}

#[test]
fn should_get_serie_genres() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let ultimate_breaks_and_beats = Series::fetch()
        .id("3e5979c8-5a78-4d0b-878a-0fb87853effe")
        .include(series::Include::Genres)
        .execute()
        .unwrap();

    assert!(ultimate_breaks_and_beats.genres.is_some());
}

#[test]
fn should_get_serie_annotation() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let record_store_day_2020 = Series::fetch()
        .id("c1071cec-48f1-4231-ac8e-8c64e15ec7cd")
        .include(series::Include::Annotation)
        .execute()
        .unwrap();

    assert!(record_store_day_2020.annotation.is_some());
}
