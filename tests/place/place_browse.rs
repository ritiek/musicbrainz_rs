extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::model::place;
use musicbrainz_rs::model::place::*;
use musicbrainz_rs::Browse;
use std::{thread, time};

// TODO : waiting for https://github.com/metabrainz/musicbrainz-server/pull/1223 to be released
// #[test]
fn should_browse_place_by_area() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let places_in_paris = Place::browse()
        .by(place::Browse::Area, "dc10c22b-e510-4006-8b7f-fecb4f36436e")
        .execute();

    println!("{:?}", places_in_paris);
    assert!(places_in_paris.is_ok());

    let places_in_paris = places_in_paris.unwrap();

    assert!(places_in_paris.count > 1);
    assert_eq!(places_in_paris.offset, 0);
    assert!(!places_in_paris.entities.is_empty());
}
