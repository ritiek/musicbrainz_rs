extern crate musicbrainz_rs;
use musicbrainz_rs::model::recording;
use musicbrainz_rs::model::recording::*;
use musicbrainz_rs::Fetch;
use std::{thread, time};

#[test]
fn should_get_recording_artists() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let association_de_gens_normal = Recording::fetch()
        .id("f5f10cee-5d84-41d0-805d-3503872c151d")
        .include(recording::Include::Artists)
        .execute();

    let artist_credit = association_de_gens_normal.unwrap().artist_credit.unwrap();

    assert!(artist_credit.iter().any(|credit| credit.name == "TTC"));
    assert!(artist_credit.iter().any(|credit| credit.name == "Svinkels"));
}

#[test]
fn should_get_recording_releases() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .include(recording::Include::Releases)
        .execute();

    let releases = you_talk_too_much.unwrap().releases;

    assert!(releases
        .unwrap()
        .iter()
        .any(|release| release.title == "Hooker ’n Heat"));
}

#[test]
fn should_get_recording_aliases() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .include(recording::Include::Aliases)
        .execute();

    let aliases = you_talk_too_much.unwrap().aliases;

    assert!(aliases.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[test]
fn should_get_recording_tags() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .include(recording::Include::Tags)
        .execute()
        .unwrap();

    assert!(you_talk_too_much.tags.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[test]
fn should_get_recording_rating() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .include(recording::Include::Rating)
        .execute()
        .unwrap();

    assert!(you_talk_too_much.rating.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[test]
fn should_get_recording_genres() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let you_talk_too_much = Recording::fetch()
        .id("de552ba4-572c-4c59-b2a9-0508619696ac")
        .include(recording::Include::Genres)
        .execute()
        .unwrap();

    assert!(you_talk_too_much.genres.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}

#[test]
fn should_get_recording_annotation() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let isolina = Recording::fetch()
        .id("2edf7653-2287-4408-8e7a-20e001a60847")
        .include(recording::Include::Annotation)
        .execute()
        .unwrap();

    assert!(isolina.annotation.is_some()); // FIXME: didn't find a recording containing actual aliases (yet)
}
