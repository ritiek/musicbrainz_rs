extern crate musicbrainz_rs;
use musicbrainz_rs::model::label;
use musicbrainz_rs::model::label::Label;
use musicbrainz_rs::Fetch;
use std::{thread, time};

#[test]
fn should_get_label_releases() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .include(label::Include::Releases)
        .execute();

    let releases = ninja_tune.unwrap().releases;

    assert!(releases
        .unwrap()
        .iter()
        .any(|release| release.title == "The Final Corporate Colonization of the Unconscious"));
}

#[test]
fn should_get_label_aliases() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let motown = Label::fetch()
        .id("8e479e57-ef44-490c-b75d-cd28df89bf1b")
        .include(label::Include::Aliases)
        .execute();

    let aliases = motown.unwrap().aliases;

    assert!(aliases.unwrap().iter().any(|alias| alias.name == "Motown"));
}

#[test]
fn should_get_label_tags() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .include(label::Include::Tags)
        .execute()
        .unwrap();

    assert!(ninja_tune
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "independent"));
}

#[test]
fn should_get_label_rating() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .include(label::Include::Rating)
        .execute()
        .unwrap();

    assert!(ninja_tune.rating.is_some());
}

#[test]
fn should_get_label_genres() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .include(label::Include::Genres)
        .execute()
        .unwrap();

    assert!(ninja_tune.genres.is_some());
}

#[test]
fn should_get_label_annotation() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let tokuma_japan_communications = Label::fetch()
        .id("040439f9-578b-45b6-b07b-d6c97e544859")
        .include(label::Include::Annotation)
        .execute()
        .unwrap();

    assert!(tokuma_japan_communications.annotation.is_some());
}
