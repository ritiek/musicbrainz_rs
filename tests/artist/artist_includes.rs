extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::model::artist;
use musicbrainz_rs::model::artist::*;
use musicbrainz_rs::Fetch;
use std::{thread, time};

#[test]
fn should_get_artist_releases() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .include(artist::Include::Releases)
        .execute()
        .unwrap();

    let releases = john_lee_hooker.releases.unwrap();

    assert!(releases
        .iter()
        .any(|release| release.title == "Boogie Chillen’ / Sally Mae"));
}

#[test]
fn should_get_artist_works() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .include(artist::Include::Works)
        .execute()
        .unwrap();

    let works = john_lee_hooker.works.unwrap();

    assert!(works
        .iter()
        .any(|work| work.title == "Baby Please Don't Go"));
}

#[test]
fn should_get_artist_release_groups() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .include(artist::Include::ReleaseGroups)
        .execute()
        .unwrap();

    let release_groups = john_lee_hooker.release_groups.unwrap();

    assert!(release_groups.iter().any(|group| group.title == "Burnin’"));
    assert!(release_groups
        .iter()
        .any(|group| group.title == "Travelin’"));
}

#[test]
fn should_get_artist_recordings() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .include(artist::Include::Recordings)
        .execute()
        .unwrap();

    let recordings = john_lee_hooker.recordings.unwrap();

    assert!(recordings
        .iter()
        .any(|recording| recording.title == "A Little Bit Higher"));
}

#[test]
fn should_get_artist_aliases() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .include(artist::Include::Aliases)
        .execute()
        .unwrap();

    let aliases = john_lee_hooker.aliases;

    assert!(aliases
        .unwrap()
        .iter()
        .any(|alias| alias.name == "Delta John"));
}

#[test]
fn should_get_artist_artist_relations() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .include(artist::Include::ArtistRelations)
        .include(artist::Include::EventRelations)
        .execute()
        .unwrap();

    let relations = john_lee_hooker.relations.unwrap();

    assert!(relations.iter().any(|rel| rel.relation_type == "parent"));
    assert!(relations
        .iter()
        .any(|rel| rel.relation_type == "main performer"));
}

#[test]
fn should_get_artist_artist_releases_with_disc_ids() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let nirvana = Artist::fetch()
        .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
        .include(artist::Include::ReleasesWithDiscIds)
        .execute()
        .unwrap();

    let releases_with_disc_ids = nirvana.releases.unwrap();

    assert!(releases_with_disc_ids
        .iter()
        .any(|release| release.title == "Smells Like Teen Spirit"));
}

#[test]
fn should_get_artist_tags() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .include(artist::Include::Tags)
        .execute()
        .unwrap();

    assert!(john_lee_hooker
        .tags
        .unwrap()
        .iter()
        .any(|tag| tag.name == "chicago blues"));
}

#[test]
fn should_get_artist_rating() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .include(artist::Include::Rating)
        .execute()
        .unwrap();

    assert!(john_lee_hooker.rating.is_some());
}

#[test]
fn should_get_artist_genres() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let john_lee_hooker = Artist::fetch()
        .id("b0122194-c49a-46a1-ade7-84d1d76bd8e9")
        .include(artist::Include::Genres)
        .execute()
        .unwrap();

    assert!(john_lee_hooker
        .genres
        .unwrap()
        .iter()
        .any(|genre| genre.name == "blues"));
}

#[test]
fn should_get_artist_annotation() {
    musicbrainz_rs::config::set_user_agent(musicbrainz_rs::config::DEFAULT_USER_AGENT);
    let franz_joseph_haydn = Artist::fetch()
        .id("c130b0fb-5dce-449d-9f40-1437f889f7fe")
        .include(artist::Include::Annotation)
        .execute()
        .unwrap();

    assert!(franz_joseph_haydn.annotation.is_some());
}
