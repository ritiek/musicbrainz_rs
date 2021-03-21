extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::model::recording;
use musicbrainz_rs::model::recording::*;
use musicbrainz_rs::Browse;
use std::{thread, time};

#[test]
fn should_browse_recording_by_artist() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let recording_by_svinkels = Recording::browse()
        .by(
            recording::Browse::Artist,
            "770d490e-c89b-4775-8508-aca7c75142cd",
        )
        .execute();

    assert!(recording_by_svinkels.is_ok());

    let recording_by_svinkels = recording_by_svinkels.unwrap();

    assert!(recording_by_svinkels.count > 1);
    assert_eq!(recording_by_svinkels.offset, 0);
    assert!(!recording_by_svinkels.entities.is_empty());
}

#[test]
fn should_browse_recording_work() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let la_javanaise_recordings = Recording::browse()
        .by(
            recording::Browse::Work,
            "578eab03-84d3-374f-a7c5-03c3a685a9a5",
        )
        .execute();

    assert!(la_javanaise_recordings.is_ok());

    let la_javanaise_recordings = la_javanaise_recordings.unwrap();

    assert!(la_javanaise_recordings.count > 1);
    assert_eq!(la_javanaise_recordings.offset, 0);
    assert!(!la_javanaise_recordings.entities.is_empty());
}

#[test]
fn should_browse_recording_by_release() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let recording_on_hooker_n_heat = Recording::browse()
        .by(
            recording::Browse::Release,
            "38860ba5-6b40-3e19-83ae-a560737a3f6f",
        )
        .execute();

    assert!(recording_on_hooker_n_heat.is_ok());

    let recording_on_hooker_n_heat = recording_on_hooker_n_heat.unwrap();

    assert!(recording_on_hooker_n_heat.count > 1);
    assert_eq!(recording_on_hooker_n_heat.offset, 0);
    assert!(!recording_on_hooker_n_heat.entities.is_empty());
}
