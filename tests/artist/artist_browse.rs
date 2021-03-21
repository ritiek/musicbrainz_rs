extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist;
use musicbrainz_rs::model::artist::*;
use musicbrainz_rs::Browse;
use std::{thread, time};

#[test]
fn should_browse_artist_by_release_groups() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let artistss_on_in_rainbows_rg = Artist::browse()
        .by(
            artist::Browse::ReleaseGroup,
            "6e335887-60ba-38f0-95af-fae7774336bf",
        )
        .execute();

    assert!(artistss_on_in_rainbows_rg.is_ok());

    let artistss_on_in_rainbows_rg = artistss_on_in_rainbows_rg.unwrap();

    assert_eq!(artistss_on_in_rainbows_rg.count, 1);
    assert_eq!(artistss_on_in_rainbows_rg.offset, 0);
    assert!(!artistss_on_in_rainbows_rg.entities.is_empty());
}

#[test]
fn should_browse_artist_by_release() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let artists_on_in_utero_release = Artist::browse()
        .by(
            artist::Browse::Release,
            "18d4e9b4-9247-4b44-914a-8ddec3502103",
        )
        .execute();

    assert!(artists_on_in_utero_release.is_ok());

    let artists_on_in_utero_release = artists_on_in_utero_release.unwrap();

    assert_eq!(artists_on_in_utero_release.count, 1);
    assert_eq!(artists_on_in_utero_release.offset, 0);
    assert!(!artists_on_in_utero_release.entities.is_empty());
}

#[test]
fn should_browse_artist_by_area() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let artistss_in_aberdeen_area = Artist::browse()
        .by(artist::Browse::Area, "a640b45c-c173-49b1-8030-973603e895b5")
        .execute();

    assert!(artistss_in_aberdeen_area.is_ok());

    let artistss_in_aberdeen_area = artistss_in_aberdeen_area.unwrap();

    assert!(artistss_in_aberdeen_area.count > 0);
    assert_eq!(artistss_in_aberdeen_area.offset, 0);
    assert!(!artistss_in_aberdeen_area.entities.is_empty());
}

#[test]
fn should_browse_artist_by_work() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let artists_on_hotel_california = Artist::browse()
        .by(artist::Browse::Work, "22457dc0-ecbf-38f5-9056-11c858530a50")
        .execute();

    let artists_on_hotel_california = artists_on_hotel_california.unwrap();

    assert!(artists_on_hotel_california.count > 1);
    assert_eq!(artists_on_hotel_california.offset, 0);
    assert!(!artists_on_hotel_california.entities.is_empty());
}

#[test]
fn should_browse_artist_by_recording() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let artists_on_polly = Artist::browse()
        .by(
            artist::Browse::Recording,
            "af40d6b8-58e8-4ca5-9db8-d4fca0b899e2",
        )
        .execute();

    let artists_on_polly = artists_on_polly.unwrap();

    assert!(artists_on_polly.count == 1);
    assert_eq!(artists_on_polly.offset, 0);
    assert!(!artists_on_polly.entities.is_empty());
}
