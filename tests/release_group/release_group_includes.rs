use musicbrainz_rs::model::release_group;
use musicbrainz_rs::model::release_group::*;
use musicbrainz_rs::Fetch;
use std::{thread, time};

#[test]
fn should_get_release_group_artists() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let harvest = ReleaseGroup::fetch()
        .id("b25419cf-71bf-3a54-8cd4-2161c61056a0")
        .include(release_group::Include::Artists)
        .execute();

    assert!(harvest
        .unwrap()
        .artist_credit
        .unwrap()
        .iter()
        .any(|credit| credit.artist.name == "Neil Young"));
}

#[test]
fn should_get_release_group_releases() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let harvest = ReleaseGroup::fetch()
        .id("b25419cf-71bf-3a54-8cd4-2161c61056a0")
        .include(release_group::Include::Releases)
        .execute();

    assert!(harvest
        .unwrap()
        .releases
        .unwrap()
        .iter()
        .any(|release| release.title == "Harvest" && release.country == Some("CA".to_string())));
}

#[test]
fn should_get_release_group_tags() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .include(release_group::Include::Tags)
        .execute()
        .unwrap();

    assert!(in_utero
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "rock_grunge"));
}

#[test]
fn should_get_release_group_aliases() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .include(release_group::Include::Aliases)
        .execute()
        .unwrap();

    assert!(in_utero.aliases.is_some());
}

#[test]
fn should_get_release_group_rating() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .include(release_group::Include::Rating)
        .execute()
        .unwrap();

    assert!(in_utero.rating.is_some());
}

#[test]
fn should_get_release_group_genres() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .include(release_group::Include::Genres)
        .execute()
        .unwrap();

    assert!(in_utero.genres.is_some());
}

#[test]
fn should_get_release_group_annotation() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let minnesoda = ReleaseGroup::fetch()
        .id("100d2ce1-8ba4-43eb-afbe-93ca21867e16")
        .include(release_group::Include::Annotation)
        .execute()
        .unwrap();

    assert!(minnesoda.annotation.is_some());
}
