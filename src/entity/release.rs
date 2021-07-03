use chrono::NaiveDate;

use super::Include;
use crate::date_format;
use crate::entity::alias::Alias;
use crate::entity::area::Area;
use crate::entity::artist_credit::ArtistCredit;
use crate::entity::coverart::CoverArtArchive;
use crate::entity::disc::Disc;
use crate::entity::genre::Genre;
use crate::entity::label::LabelInfo;
use crate::entity::recording::Recording;
use crate::entity::relations::Relation;
use crate::entity::release_group::ReleaseGroup;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;

/// A MusicBrainz release represents the unique release (i.e. issuing) of a product on a specific
/// date with specific release information such as the country, label, barcode and packaging.
/// If you walk into a store and purchase an album or single, they are each represented in
/// MusicBrainz as one release.
///
/// Each release belongs to a release group and contains at least one medium (commonly referred to
/// as a disc when talking about a CD release). Each medium has a tracklist.
/// A medium is the actual physical medium that stores the audio content. This means that each CD
/// in a multi-disc release will be entered as separate mediums within the release, and that both
/// sides of a vinyl record or cassette will exist on one medium. Mediums have a format (e.g. CD,
/// DVD, vinyl, and cassette) and can optionally also have a title. Sometimes a medium can be a
/// side of a disc. For example, the two sides of a hybrid SACD (the CD side and the SACD side)
/// should be entered as two mediums.
/// Tracklists represent the set and ordering of tracks as listed on a liner, and the same tracklist
/// can appear on more than one release. For example, a boxset compilation that contains previously
/// released CDs would share the same tracklists as the separate releases.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Release {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,

    /// The title of the release.
    pub title: String,

    #[serde(rename = "status-id")]
    pub status_id: Option<String>,

    /// The status describes how "official" a release is.
    pub status: Option<ReleaseStatus>,

    /// The date the release was issued.
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub date: Option<NaiveDate>,

    /// The country the release was issued in.
    pub country: Option<String>,

    ///  Data quality indicates how good the data for a release is. It is not a mark of how good or
    /// bad the music itself is - for that, use ratings.
    pub quality: Option<ReleaseQuality>,

    pub barcode: Option<String>,

    pub disambiguation: Option<String>,

    #[serde(rename = "packaging-id")]
    pub packaging_id: Option<String>,

    /// The physical packaging that accompanies the release. See the
    /// [list of packaging](https://musicbrainz.org/doc/Release/Packaging) for more information.
    pub packaging: Option<String>, //TODO: This might be an enum needs to test all against all possible values

    pub relations: Option<Vec<Relation>>,
    pub release_group: Option<ReleaseGroup>,
    pub artist_credit: Option<Vec<ArtistCredit>>,
    pub media: Option<Vec<Media>>,
    pub label_info: Option<Vec<LabelInfo>>,
    pub tags: Option<Vec<Tag>>,
    pub aliases: Option<Vec<Alias>>,
    pub genres: Option<Vec<Genre>>,
    pub annotation: Option<String>,
    pub asin: Option<String>,
    #[serde(rename = "cover-art-archive")]
    pub cover_art_archive: Option<CoverArtArchive>,
    #[serde(rename = "text-representation")]
    pub text_representation: Option<ReleaseTextRepresentation>,
    pub release_events: Option<Vec<ReleaseEvent>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ReleaseTextRepresentation {
    pub language: Language,
    pub script: ReleaseScript,
}

/// The script used to write the release's track list. The possible values are taken from the
/// [ISO 15924](https://en.wikipedia.org/wiki/ISO_15924) standard.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ReleaseScript {
    /* TODO: we need to test all posible values to build the enum see https://musicbrainz.org/doc/Release */
    /// ## Latin (also known as Roman or, incorrectly, "English")
    /// Latin is the most common script, and usually the correct choice. It is used
    /// for all Western European languages, and many others. It is also the most common script used for transliterations.
    Latn,
}

/* TODO: we need to test all posible values to build the enum see https://musicbrainz.org/doc/Release */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum Language {
    Eng,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum ReleaseQuality {
    /// The release needs serious fixes, or its existence is hard to prove (but it's not clearly fake).
    Low,

    /// All available data has been added, if possible including cover art with liner info that
    /// proves it.
    High,

    /// This is the default setting - technically "unknown" if the quality has never been modified,
    /// "normal" if it has.
    Normal,

    Unknown,

    None,
}

/// The release status describes how "official" a release is.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ReleaseStatus {
    /// Any release officially sanctioned by the artist and/or their record company. Most releases
    /// will fit into this category.
    Official,

    /// A give-away release or a release intended to promote an upcoming official release (e.g.
    /// pre-release versions, releases included with a magazine, versions supplied to radio DJs
    /// for air-play).
    Promotion,

    /// An unofficial/underground release that was not sanctioned by the artist and/or the record
    /// company. This includes unofficial live recordings and pirated releases.
    Bootleg,

    /// An alternate version of a release where the titles have been changed. These don't correspond
    /// to any real release and should be linked to the original release using the
    /// [transl(iter)ation relationship](https://musicbrainz.org/relationship/fc399d47-23a7-4c28-bfcf-0607a562b644).
    PseudoRelease,

    None,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct ReleaseEvent {
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    date: Option<NaiveDate>,
    area: Area,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Media {
    pub title: String,
    pub position: u32,
    pub track_count: u32,
    pub disc_count: Option<u32>,
    pub format_id: Option<String>,
    pub format: Option<String>,
    pub tracks: Option<Vec<Track>>,
    pub discs: Option<Vec<Disc>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Track {
    pub recording: Recording,
    pub title: String,
    pub number: String,
    pub length: Option<u32>,
    pub position: u32,
    pub id: String,
}

impl_browse! {
Release,
   (by_area, BrowseBy::Area),
   (by_artist, BrowseBy::Artist),
   (by_label, BrowseBy::Label),
   (by_track, BrowseBy::Track),
   (by_track_artist, BrowseBy::TrackArtist),
   (by_recording, BrowseBy::Recording),
   (by_release_group, BrowseBy::ReleaseGroup),
   (by_collection, BrowseBy::Collection)
}

impl_includes!(
    Release,
    (with_artists, Include::Artists),
    (with_labels, Include::Labels),
    (with_artist_relations, Include::ArtistRelations),
    (with_recordings, Include::Recordings),
    (with_release_groups, Include::ReleaseGroups),
    (with_tags, Include::Tags),
    (with_ratings, Include::Rating),
    (with_aliases, Include::Aliases),
    (with_genres, Include::Genres),
    (with_annotations, Include::Annotations)
);
