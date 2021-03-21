extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::model::event;
use musicbrainz_rs::model::event::*;
use musicbrainz_rs::Browse;
use std::{thread, time};

#[test]
fn should_browse_event_by_place() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let events_in_north_stage_woodstock_1994 = Event::browse()
        .by(event::Browse::Place, "380bad3f-d3d7-4a1c-9e7f-c6ec2661165c")
        .execute();

    assert!(events_in_north_stage_woodstock_1994.is_ok());

    let events_in_north_stage_woodstock_1994 = events_in_north_stage_woodstock_1994.unwrap();

    assert!(events_in_north_stage_woodstock_1994.count > 1);
    assert_eq!(events_in_north_stage_woodstock_1994.offset, 0);
    assert!(!events_in_north_stage_woodstock_1994.entities.is_empty());
}

#[test]
fn should_browse_event_by_artist() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let events_with_aerosmith = Event::browse()
        .by(
            event::Browse::Artist,
            "3d2b98e5-556f-4451-a3ff-c50ea18d57cb",
        )
        .execute();

    assert!(events_with_aerosmith.is_ok());

    let events_with_aerosmith = events_with_aerosmith.unwrap();

    assert!(events_with_aerosmith.count > 1);
    assert_eq!(events_with_aerosmith.offset, 0);
    assert!(!events_with_aerosmith.entities.is_empty());
}

#[test]
fn should_browse_event_by_area() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let events_in_montreux = Event::browse()
        .by(event::Browse::Area, "d872ed01-edfd-4b39-8ab5-f8b3c84fc001")
        .execute();

    assert!(events_in_montreux.is_ok());

    let events_in_montreux = events_in_montreux.unwrap();

    assert!(events_in_montreux.count > 1);
    assert_eq!(events_in_montreux.offset, 0);
    assert!(!events_in_montreux.entities.is_empty());
}
