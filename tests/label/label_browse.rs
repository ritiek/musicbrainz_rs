extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::model::label;
use musicbrainz_rs::model::label::*;
use musicbrainz_rs::Browse;
use std::{thread, time};

#[test]
fn should_browse_label_by_area() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let labels_in_paris = Label::browse()
        .by(label::Browse::Area, "dc10c22b-e510-4006-8b7f-fecb4f36436e")
        .execute();

    assert!(labels_in_paris.is_ok());

    let labels_in_paris = labels_in_paris.unwrap();

    assert!(labels_in_paris.count > 1);
    assert_eq!(labels_in_paris.offset, 0);
    assert!(!labels_in_paris.entities.is_empty());
}

#[test]
fn should_browse_label_by_release() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let label_of_justice_cross_release = Label::browse()
        .by(
            label::Browse::Release,
            "4642ee19-7790-3c8d-ab5e-d133de942db6",
        )
        .execute();

    assert!(label_of_justice_cross_release.is_ok());

    let label_of_justice_cross_release = label_of_justice_cross_release.unwrap();

    assert!(label_of_justice_cross_release.count > 1);
    assert_eq!(label_of_justice_cross_release.offset, 0);
    assert!(!label_of_justice_cross_release.entities.is_empty());
}
