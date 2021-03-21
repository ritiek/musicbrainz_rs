extern crate chrono;
extern crate musicbrainz_rs;

use chrono::NaiveDate;

use musicbrainz_rs::model::area::*;
use musicbrainz_rs::model::artist::ArtistType::*;
use musicbrainz_rs::model::artist::*;
use musicbrainz_rs::model::event::Event;
use musicbrainz_rs::model::instrument::*;
use musicbrainz_rs::model::label::*;
use musicbrainz_rs::model::lifespan::*;
use musicbrainz_rs::model::place::*;
use musicbrainz_rs::model::recording::Recording;
use musicbrainz_rs::model::release::*;
use musicbrainz_rs::model::release_group::*;
use musicbrainz_rs::model::series::*;
use musicbrainz_rs::model::url::*;
use musicbrainz_rs::model::work::*;
use musicbrainz_rs::Fetch;
use std::{thread, time};

#[test]
fn should_get_artist_by_id() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .execute();

    assert_eq!(
        nirvana.unwrap(),
        Artist {
            id: String::from("5b11f4ce-a62d-471e-81fc-a69a8278c7da"),
            name: String::from("Nirvana"),
            sort_name: String::from("Nirvana"),
            disambiguation: String::from("90s US grunge band"),
            artist_type: Some(Group),
            gender: None,
            country: Some("US".to_string()),
            area: Some(Area {
                id: "489ce91b-6658-3307-9877-795b68554c98".to_string(),
                area_type: None,
                type_id: None,
                disambiguation: "".to_string(),
                name: "United States".to_string(),
                sort_name: "United States".to_string(),
                iso_3166_1_codes: Some(vec!["US".to_string(),]),
                life_span: None,
                tags: None,
                aliases: None,
                genres: None,
                annotation: None,
            }),
            begin_area: Some(Area {
                id: "a640b45c-c173-49b1-8030-973603e895b5".to_string(),
                area_type: None,
                type_id: None,
                disambiguation: "".to_string(),
                name: "Aberdeen".to_string(),
                sort_name: "Aberdeen".to_string(),
                iso_3166_1_codes: None,
                life_span: None,
                tags: None,
                aliases: None,
                genres: None,
                annotation: None,
            }),
            life_span: Some(LifeSpan {
                ended: Some(true),
                begin: Some(NaiveDate::from_ymd(1988, 1, 1)),
                end: Some(NaiveDate::from_ymd(1994, 4, 5)),
            }),
            tags: None,
            relations: None,
            releases: None,
            recordings: None,
            release_groups: None,
            works: None,
            aliases: None,
            rating: None,
            genres: None,
            annotation: None,
        }
    );
}

#[test]
fn should_get_recording_by_id() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let polly = Recording::fetch()
        .id("af40d6b8-58e8-4ca5-9db8-d4fca0b899e2")
        .execute();

    assert_eq!(
        polly.unwrap(),
        Recording {
            id: "af40d6b8-58e8-4ca5-9db8-d4fca0b899e2".to_string(),
            title: "(New Wave) Polly".to_string(),
            video: Some(false),
            length: Some(246_000),
            disambiguation: Some("".to_string()),
            aliases: None,
            artist_credit: None,
            relations: None,
            releases: None,
            tags: None,
            rating: None,
            genres: None,
            annotation: None,
        }
    );
}

#[test]
fn should_get_release_group_by_id() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let in_utero = ReleaseGroup::fetch()
        .id("2a0981fb-9593-3019-864b-ce934d97a16e")
        .execute();

    assert_eq!(
        in_utero.unwrap(),
        ReleaseGroup {
            id: "2a0981fb-9593-3019-864b-ce934d97a16e".to_string(),
            primary_type_id: "f529b476-6e62-324f-b0aa-1f3e33d313fc".to_string(),
            primary_type: "Album".to_string(),
            secondary_type_ids: vec![],
            secondary_types: vec![],
            first_release_date: NaiveDate::from_ymd(1993, 9, 21),
            title: "In Utero".to_string(),
            disambiguation: "".to_string(),
            artist_credit: None,
            releases: None,
            tags: None,
            aliases: None,
            rating: None,
            genres: None,
            annotation: None,
        }
    );
}

#[test]
fn should_get_release() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let in_utero = Release::fetch()
        .id("18d4e9b4-9247-4b44-914a-8ddec3502103")
        .execute();

    assert_eq!(
        in_utero.unwrap(),
        Release {
            id: "18d4e9b4-9247-4b44-914a-8ddec3502103".to_string(),
            title: "In Utero".to_string(),
            status_id: Some("4e304316-386d-3409-af2e-78857eec5cfe".to_string()),
            status: Some(ReleaseStatus::Official),
            date: Some(NaiveDate::from_ymd(1993, 1, 1)),
            country: Some("US".to_string()),
            quality: Some(ReleaseQuality::Normal),
            barcode: Some("0208314671259".to_string()),
            disambiguation: Some("".to_string()),
            packaging_id: Some("ec27701a-4a22-37f4-bfac-6616e0f9750a".to_string()),
            packaging: Some("Jewel Case".to_string()),
            label_info: None,
            media: None,
            release_group: None,
            tags: None,
            aliases: None,
            genres: None,
            annotation: None,
        }
    );
}

#[test]
fn should_get_work_by_id() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let hotel_california = Work::fetch()
        .id("22457dc0-ecbf-38f5-9056-11c858530a50")
        .execute();

    assert_eq!(
        hotel_california.unwrap(),
        Work {
            id: "22457dc0-ecbf-38f5-9056-11c858530a50".to_string(),
            title: "Hotel California".to_string(),
            type_id: Some("f061270a-2fd6-32f1-a641-f0f8676d14e6".to_string()),
            work_type: Some("Song".to_string()),
            languages: Some(vec!["eng".to_string()]),
            language: Some("eng".to_string()),
            disambiguation: Some("".to_string()),
            tags: None,
            aliases: None,
            rating: None,
            genres: None,
            annotation: None,
        }
    );
}

#[test]
fn should_get_label_by_id() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .execute();

    assert_eq!(
        ninja_tune.unwrap(),
        Label {
            id: "dc940013-b8a8-4362-a465-291026c04b42".to_string(),
            type_id: Some("7aaa37fe-2def-3476-b359-80245850062d".to_string()),
            label_type: Some("Original Production".to_string()),
            name: "Ninja Tune".to_string(),
            sort_name: "Ninja Tune".to_string(),
            disambiguation: "".to_string(),
            country: Some("GB".to_string()),
            label_code: Some(12885),
            releases: None,
            aliases: None,
            tags: None,
            rating: None,
            genres: None,
            annotation: None,
        }
    );
}

#[test]
fn should_get_area_by_id() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let aberdeen = Area::fetch()
        .id("a640b45c-c173-49b1-8030-973603e895b5")
        .execute();

    assert_eq!(
        aberdeen.unwrap(),
        Area {
            id: "a640b45c-c173-49b1-8030-973603e895b5".to_string(),
            area_type: Some("City".to_string()),
            type_id: Some("6fd8f29a-3d0a-32fc-980d-ea697b69da78".to_string()),
            disambiguation: "".to_string(),
            name: "Aberdeen".to_string(),
            sort_name: "Aberdeen".to_string(),
            iso_3166_1_codes: None,
            tags: None,
            aliases: None,
            genres: None,
            annotation: None,
            life_span: Some(LifeSpan {
                ended: Some(false),
                begin: None,
                end: None,
            })
        }
    );
}

#[test]
fn should_get_event_by_id() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let dour_festival_1989 = Event::fetch()
        .id("73df2f48-383b-4930-bad3-05ba938be578")
        .execute();

    assert_eq!(
        dour_festival_1989.unwrap(),
        Event {
            id: "73df2f48-383b-4930-bad3-05ba938be578".to_string(),
            name: "Dour 1989".to_string(),
            cancelled: false,
            type_id: Some("b6ded574-b592-3f0e-b56e-5b5f06aa0678".to_string()),
            event_type: Some("Festival".to_string()),
            disambiguation: "".to_string(),
            time: "".to_string(),
            setlist: "".to_string(),
            life_span: Some(LifeSpan {
                begin: Some(NaiveDate::from_ymd(1989, 9, 16)),
                end: Some(NaiveDate::from_ymd(1989, 9, 16)),
                ended: Some(true)
            }),
            tags: None,
            aliases: None,
            rating: None,
            genres: None,
            annotation: None,
        }
    );
}

#[test]
fn should_get_instrument() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let mandoline = Instrument::fetch()
        .id("37fa9bb5-d5d7-4b0f-aa4d-531339ba9c32")
        .execute();

    assert_eq!(
        mandoline.unwrap(),
        Instrument {
            id: "37fa9bb5-d5d7-4b0f-aa4d-531339ba9c32".to_string(),
            name: "mandolin".to_string(),
            instrument_type: "String instrument".to_string(),
            type_id: "cc00f97f-cf3d-3ae2-9163-041cb1a0d726".to_string(),
            description: "".to_string(),
            disambiguation: "".to_string(),
            tags: None,
            aliases: None,
            genres: None,
            annotation: None,
        }
    );
}

// TODO : waiting for https://github.com/metabrainz/musicbrainz-server/pull/1223 to be released
// #[test]
fn should_get_place() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let blue_note_record = Place::fetch()
        .id("327c29c6-da63-4dc9-a117-1917ee691ce4")
        .execute();

    assert_eq!(
        blue_note_record.unwrap(),
        Place {
            id: "327c29c6-da63-4dc9-a117-1917ee691ce4".to_string(),
            name: "Blue Note".to_string(),
            disambiguation: "Chicago, 1954-1960".to_string(),
            life_span: LifeSpan {
                begin: Some(NaiveDate::from_ymd(1954, 4, 2)),
                end: Some(NaiveDate::from_ymd(1960, 6, 14)),
                ended: Some(true)
            },
            type_id: Some("cd92781a-a73f-30e8-a430-55d7521338db".to_string()),
            place_type: Some("Venue".to_string()),
            address: "3 North Clark Street, Chicago, IL 60602".to_string(),
            area: Area {
                id: "29a709d8-0320-493e-8d0c-f2c386662b7f".to_string(),
                disambiguation: "".to_string(),
                sort_name: "Chicago".to_string(),
                name: "Chicago".to_string(),
                area_type: None,
                type_id: None,
                iso_3166_1_codes: None,
                life_span: None,
                tags: None,
                aliases: None,
                genres: None,
                annotation: None,
            },
            coordinates: Some(Coordinates {
                latitude: 41.882_059,
                longitude: -87.630_881,
            }),
            aliases: None,
            tags: None,
            genres: None,
            annotation: None,
        }
    );
}

#[test]
fn should_get_series() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let la_chanson_du_dimanche = Series::fetch()
        .id("814fb4d5-327f-4e37-8784-f8a707e5f97c")
        .execute();

    assert_eq!(
        la_chanson_du_dimanche.unwrap(),
        Series {
            id: "814fb4d5-327f-4e37-8784-f8a707e5f97c".to_string(),
            type_id: "dd968243-7128-30a2-81f0-79843430a8e2".to_string(),
            serie_type: "Recording series".to_string(),
            disambiguation: "".to_string(),
            name: "La Chanson du Dimanche — Saison 4".to_string(),
            tags: None,
            aliases: None,
            genres: None,
            annotation: None,
        }
    );
}

#[test]
fn should_get_url() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let svinkels_dot_com = Url::fetch()
        .id("9237f6da-fec6-4b8a-9d52-c7c18e0e2630")
        .execute();

    assert_eq!(
        svinkels_dot_com.unwrap(),
        Url {
            resource: "http://www.svinkels.com/".to_string(),
            id: "9237f6da-fec6-4b8a-9d52-c7c18e0e2630".to_string(),
            tags: None,
        }
    );
}
