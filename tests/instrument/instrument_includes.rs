extern crate musicbrainz_rs;
use musicbrainz_rs::model::instrument;
use musicbrainz_rs::model::instrument::Instrument;
use musicbrainz_rs::Fetch;
use std::{thread, time};

#[test]
fn should_get_instrument_tags() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let guitar = Instrument::fetch()
        .id("63021302-86cd-4aee-80df-2270d54f4978")
        .include(instrument::Include::Tags)
        .execute()
        .unwrap();

    assert!(guitar.tags.unwrap().iter().any(|tag| tag.name == "wood"));
}

#[test]
fn should_get_instrument_aliases() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let guitar = Instrument::fetch()
        .id("63021302-86cd-4aee-80df-2270d54f4978")
        .include(instrument::Include::Aliases)
        .execute()
        .unwrap();

    assert!(guitar
        .aliases
        .unwrap()
        .iter()
        .any(|alias| alias.name == "guitarras"));
}

#[test]
fn should_get_instrument_genres() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let guitar = Instrument::fetch()
        .id("63021302-86cd-4aee-80df-2270d54f4978")
        .include(instrument::Include::Genres)
        .execute()
        .unwrap();

    assert!(guitar.genres.is_some());
}

#[test]
fn should_get_instrument_annotation() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let gusli = Instrument::fetch()
        .id("bb08cebd-ff6c-49e8-8f8f-914cc2d68c27")
        .include(instrument::Include::Annotation)
        .execute()
        .unwrap();

    assert!(gusli.annotation.is_some());
}
