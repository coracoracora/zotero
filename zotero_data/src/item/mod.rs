//! # All Zotero data structures and their builders
//!
//! This structs are used for serialize and deserialize zotero Item's data.
//! This structs can also be used to create or update zotero Item's data.
//!
//! ```rust
//! use zotero_data::item::{Creator, CreatorBuilder, BookData, BookDataBuilder};
//!
//! let creators : Vec<Creator> = vec![
//!     CreatorBuilder::default()
//!         .creator_type("author")
//!         .first_name("John")
//!         .last_name("Doe")
//!         .build()
//!         .unwrap()
//! ];
//!
//! let new_book : BookData = BookDataBuilder::default()
//!       .title("Book title")
//!       .creators(creators)
//!       .item_type("book")
//!       .build()
//!       .unwrap();
//! ```

mod item_data;

use chrono::{DateTime, NaiveTime};
use chrono::{Local, NaiveDate, NaiveDateTime};
pub use item_data::ArtworkData;
pub use item_data::ArtworkDataBuilder;
pub use item_data::AttachmentData;
pub use item_data::AttachmentDataBuilder;
pub use item_data::AudioRecordingData;
pub use item_data::AudioRecordingDataBuilder;
pub use item_data::BillData;
pub use item_data::BillDataBuilder;
pub use item_data::BlogPostData;
pub use item_data::BlogPostDataBuilder;
pub use item_data::BookData;
pub use item_data::BookDataBuilder;
pub use item_data::BookSectionData;
pub use item_data::BookSectionDataBuilder;
pub use item_data::CaseData;
pub use item_data::CaseDataBuilder;
pub use item_data::ComputerProgramData;
pub use item_data::ComputerProgramDataBuilder;
pub use item_data::ConferencePaperData;
pub use item_data::ConferencePaperDataBuilder;
pub use item_data::DictionaryEntryData;
pub use item_data::DictionaryEntryDataBuilder;
pub use item_data::DocumentData;
pub use item_data::DocumentDataBuilder;
pub use item_data::EmailData;
pub use item_data::EmailDataBuilder;
pub use item_data::EncyclopediaArticleData;
pub use item_data::EncyclopediaArticleDataBuilder;
pub use item_data::FilmData;
pub use item_data::FilmDataBuilder;
pub use item_data::ForumPostData;
pub use item_data::ForumPostDataBuilder;
pub use item_data::HearingData;
pub use item_data::HearingDataBuilder;
pub use item_data::InstantMessageData;
pub use item_data::InstantMessageDataBuilder;
pub use item_data::InterviewData;
pub use item_data::InterviewDataBuilder;
pub use item_data::JournalArticleData;
pub use item_data::JournalArticleDataBuilder;
pub use item_data::LetterData;
pub use item_data::LetterDataBuilder;
pub use item_data::MagazineArticleData;
pub use item_data::MagazineArticleDataBuilder;
pub use item_data::ManuscriptData;
pub use item_data::ManuscriptDataBuilder;
pub use item_data::MapData;
pub use item_data::MapDataBuilder;
pub use item_data::NewspaperArticleData;
pub use item_data::NewspaperArticleDataBuilder;
pub use item_data::NoteData;
pub use item_data::NoteDataBuilder;
pub use item_data::PatentData;
pub use item_data::PatentDataBuilder;
pub use item_data::PodcastData;
pub use item_data::PodcastDataBuilder;
pub use item_data::PresentationData;
pub use item_data::PresentationDataBuilder;
pub use item_data::RadioBroadcastData;
pub use item_data::RadioBroadcastDataBuilder;
pub use item_data::ReportData;
pub use item_data::ReportDataBuilder;
pub use item_data::StatuteData;
pub use item_data::StatuteDataBuilder;
pub use item_data::ThesisData;
pub use item_data::ThesisDataBuilder;
pub use item_data::TvBroadcastData;
pub use item_data::TvBroadcastDataBuilder;
pub use item_data::VideoRecordingData;
pub use item_data::VideoRecordingDataBuilder;
pub use item_data::WebpageData;
pub use item_data::WebpageDataBuilder;
use once_cell::sync::Lazy;
use regex::Regex;

use serde::Deserialize;
use serde::Serialize;

use crate::shared_fields::{ItemCommon, Library, Links, Tag};

use derive_builder::Builder;

use zotero_derive::ItemCommon;

#[derive(Deserialize, Serialize, Debug, Clone, ItemCommon)]
#[serde(rename_all = "camelCase", tag = "itemType")]
/// An enum that holds structs used to deserialize zotero item data into rust structs.
pub enum ItemType {
    /// A piece of artwork (e.g., an oil painting, photograph, or sculpture). Also use this item type for other types of images or visual items (e.g., scientific figures).
    Artwork(ArtworkData),
    /// Any form of audio recording, including music, spoken word, sound effects, archival recordings, or audio-based scientific figures.
    AudioRecording(AudioRecordingData),
    /// A proposed piece of legislation.
    Bill(BillData),
    /// An article or entry posted to a personal blog website. For online articles published as part of a larger online publication (e.g., NYT BlogsData), using Magazine Article or Newspaper Article generally yields better results
    BlogPost(BlogPostData),
    /// A book or similar published item. For government documents, technical reports, manuals, etc., use Report instead. This item type can also be adapted to fit many types of unusual items.
    Book(BookData),
    /// A section of a book. Usually chapters, but also forewords, prefaces, introductions, appendices, afterwords, comments, etc.
    BookSection(BookSectionData),
    /// A legal case, either published or unpublished.
    Case(CaseData),
    /// A piece of software or other computer program.
    ComputerProgram(ComputerProgramData),
    /// A paper presented at a conference and subsequently published in a formal conference proceedings publication (e.g., as a book, report, or issue of a journal). For conference papers that have not been published in a proceedings, use Presentation.
    ConferencePaper(ConferencePaperData),
    /// An entry published as part of a dictionary.
    DictionaryEntry(DictionaryEntryData),
    /// A generic document item. This item type has a poor selection of fields and poor support in citation styles, so it should generally be avoided.
    Document(DocumentData),
    /// A message sent via email. This type could also be used for other forms of personal communication.
    Email(EmailData),
    /// An article or chapter published as part of an encyclopedia.
    EncyclopediaArticle(EncyclopediaArticleData),
    ///  A film or motion picture. Generally, use this type for artistically-oriented films (including fictional, non-fictional, and documentary films). For other types of video items, use Video Recording.
    Film(FilmData),
    /// A post on an online discussion forum. Also use this type for items such as Facebook posts or tweets.
    ForumPost(ForumPostData),
    /// A formal hearing or meeting report by a legislative body.
    Hearing(HearingData),
    /// A message sent via an instant message or chat service. This type could also be used for other forms of personal communication.
    InstantMessage(InstantMessageData),
    /// An interview with a person, including recordings, transcripts, or other records of the interview.
    Interview(InterviewData),
    /// An article published in a scholarly journal (either print or online).
    JournalArticle(JournalArticleData),
    /// A letter sent between persons or organizations. This type could also be used for other forms of personal communication.
    Letter(LetterData),
    /// An article published in a non-scholarly, popular, or trade magazine (either print or online).
    MagazineArticle(MagazineArticleData),
    /// An unpublished manuscript. Use this type for both historical documents and modern unpublished work (e.g., unpublished manuscripts, manuscripts submitted for publication, working papers that are not widely available). Can also be used for other forms of historical or archival documents. This item type can also be adapted to fit many types of unusual items.
    Manuscript(ManuscriptData),
    /// A map. Also use this type for geographic models.
    Map(MapData),
    /// An article published in a newspaper (either print or online).
    NewspaperArticle(NewspaperArticleData),
    /// A patent awarded for an invention.
    Patent(PatentData),
    /// A podcast (an episode of an audio or video program distributed online, often via subscription).
    Podcast(PodcastData),
    /// A presentation made as part of a conference, meeting, symposium, lecture, etc. This item type refers to the presentation itself, not a written version published as part of a conference proceedings (use Conference Paper for such published versions).
    Presentation(PresentationData),
    /// An audio broadcast, such as a radio news show, an episode of a radio entertainment series, or similar. Includes broadcasts from online radio stations and audio broadcasts archived online (cf. Podcast).
    RadioBroadcast(RadioBroadcastData),
    /// A report published by an organization, institution, government department, or similar entity. Also used for working papers and preprints distributed through institutional repositories or preprint servers. This item type can also be adapted to fit many types of unusual items.
    Report(ReportData),
    /// A law or other piece of enacted legislation.
    Statute(StatuteData),
    /// A thesis submitted as part of a student applying for a degree (either published or unpublished).
    Thesis(ThesisData),
    /// An episode of a television series.
    TvBroadcast(TvBroadcastData),
    /// A video recording. Use this type for general video items that do not fit into one of the more specific video item types (e.g., Film, TV BroadcastData), such as YouTube videos or video-based scientific figures.
    VideoRecording(VideoRecordingData),
    /// An online page of a website. When possible, use one of the more specific item types above (e.g., Magazine Article, Blog Post, Report).
    Webpage(WebpageData),
    /// A standalone attachment file (e.g., a PDF, JPEG, DOCX, PPTX, XLSX, or ODT file). Standalone attachment files have limited functionality in Zotero (e.g., they cannot be properly searched or cited). Always attach files to proper Zotero items.
    Attachment(AttachmentData),
    /// A standalone note. Notes can be used for organizing and annotating in Zotero. If you cite a standalone note, Zotero will use the first 120 characters as the item title (and will treat the note as an author-less and date-less item). Citing notes is not a reliable way to add standalone commentary to a bibliography or reference list.
    Note(NoteData),
}

/// A struct used to represent or deserialize zotero items into rust struct
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Item {
    pub key: String,
    pub version: usize,
    pub library: Library,
    pub links: Links,
    pub meta: ItemMeta,
    pub data: ItemType,
}

impl Item {
    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn title(&self) -> &str {
        self.data.title()
    }

    pub fn tags(&self) -> &Vec<Tag> {
        self.data.tags()
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags().iter().any(|t| t.tag == tag)
    }

    //author function can not be implement for all structs automatically, fields do not exists everywhere
    pub fn author(&self) -> String {
        match &self.data {
            ItemType::Artwork(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Book(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Document(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::JournalArticle(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Report(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::MagazineArticle(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Map(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Letter(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Statute(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::EncyclopediaArticle(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Bill(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Case(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Hearing(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::ConferencePaper(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::ForumPost(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Webpage(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::NewspaperArticle(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::RadioBroadcast(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::InstantMessage(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::DictionaryEntry(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Presentation(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Manuscript(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::BlogPost(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::TvBroadcast(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Patent(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Email(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::VideoRecording(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Thesis(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::ComputerProgram(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Podcast(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::AudioRecording(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::BookSection(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Interview(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Film(d) => d
                .creators
                .iter()
                .map(|c| c.full_name())
                .collect::<Vec<String>>()
                .join(", "),
            ItemType::Attachment(_) => {
                //Attachments are mostly Pdf with parents, for the author you need to call author() on the parent!
                "".to_string()
            }
            ItemType::Note(_) => "You".to_string(),
        }
    }

    //author function can not be implement for all structs automatically, fields do not exists everywhere
    pub fn date(&self) -> DateTime<Local> {
        let date_str = match &self.data {
            ItemType::Artwork(d) => &d.date,
            ItemType::Book(d) => &d.date,
            ItemType::Document(d) => &d.date,
            ItemType::JournalArticle(d) => &d.date,
            ItemType::Report(d) => &d.date,
            ItemType::MagazineArticle(d) => &d.date,
            ItemType::Map(d) => &d.date,
            ItemType::Letter(d) => &d.date,
            ItemType::Statute(d) => &d.date_enacted,
            ItemType::EncyclopediaArticle(d) => &d.date,
            ItemType::Bill(d) => &d.date,
            ItemType::Case(d) => &d.date_decided,
            ItemType::Hearing(d) => &d.date,
            ItemType::ConferencePaper(d) => &d.date,
            ItemType::ForumPost(d) => &d.date,
            ItemType::Webpage(d) => &d.date,
            ItemType::NewspaperArticle(d) => &d.date,
            ItemType::RadioBroadcast(d) => &d.date,
            ItemType::InstantMessage(d) => &d.date,
            ItemType::DictionaryEntry(d) => &d.date,
            ItemType::Presentation(d) => &d.date,
            ItemType::Manuscript(d) => &d.date,
            ItemType::BlogPost(d) => &d.date,
            ItemType::TvBroadcast(d) => &d.date,
            ItemType::Patent(d) => &d.issue_date,
            ItemType::Email(d) => &d.date,
            ItemType::VideoRecording(d) => &d.date,
            ItemType::Thesis(d) => &d.date,
            ItemType::ComputerProgram(d) => &d.date,
            ItemType::Podcast(d) => &d.access_date,
            ItemType::AudioRecording(d) => &d.date,
            ItemType::BookSection(d) => &d.access_date,
            ItemType::Interview(d) => &d.date,
            ItemType::Film(d) => &d.date,
            ItemType::Attachment(d) => &d.date_added,
            ItemType::Note(d) => &d.date_added,
        };
        convert_zotero_date_str(date_str)
    }

    pub fn parent_item(&self) -> Option<&str> {
        Some(match &self.data {
            ItemType::Attachment(d) => &d.parent_item,
            ItemType::Note(d) => &d.parent_item,
            _ => return None,
        })
    }

    pub fn abstract_note(&self) -> Option<&str> {
        Some(match &self.data {
            ItemType::Artwork(d) => &d.abstract_note,
            ItemType::AudioRecording(d) => &d.abstract_note,
            ItemType::Bill(d) => &d.abstract_note,
            ItemType::BlogPost(d) => &d.abstract_note,
            ItemType::Book(d) => &d.abstract_note,
            ItemType::BookSection(d) => &d.abstract_note,
            ItemType::Case(d) => &d.abstract_note,
            ItemType::ComputerProgram(d) => &d.abstract_note,
            ItemType::ConferencePaper(d) => &d.abstract_note,
            ItemType::DictionaryEntry(d) => &d.abstract_note,
            ItemType::Document(d) => &d.abstract_note,
            ItemType::Email(d) => &d.abstract_note,
            ItemType::EncyclopediaArticle(d) => &d.abstract_note,
            ItemType::Film(d) => &d.abstract_note,
            ItemType::ForumPost(d) => &d.abstract_note,
            ItemType::Hearing(d) => &d.abstract_note,
            ItemType::InstantMessage(d) => &d.abstract_note,
            ItemType::Interview(d) => &d.abstract_note,
            ItemType::JournalArticle(d) => &d.abstract_note,
            ItemType::Letter(d) => &d.abstract_note,
            ItemType::MagazineArticle(d) => &d.abstract_note,
            ItemType::Manuscript(d) => &d.abstract_note,
            ItemType::Map(d) => &d.abstract_note,
            ItemType::NewspaperArticle(d) => &d.abstract_note,
            ItemType::Patent(d) => &d.abstract_note,
            ItemType::Podcast(d) => &d.abstract_note,
            ItemType::Presentation(d) => &d.abstract_note,
            ItemType::RadioBroadcast(d) => &d.abstract_note,
            ItemType::Report(d) => &d.abstract_note,
            ItemType::Statute(d) => &d.abstract_note,
            ItemType::Thesis(d) => &d.abstract_note,
            ItemType::TvBroadcast(d) => &d.abstract_note,
            ItemType::VideoRecording(d) => &d.abstract_note,
            ItemType::Webpage(d) => &d.abstract_note,
            _ => return None,
        })
    }

    pub fn publication_title(&self) -> Option<&str> {
        Some(match &self.data {
            ItemType::JournalArticle(d) => &d.publication_title,
            ItemType::MagazineArticle(d) => &d.publication_title,
            ItemType::NewspaperArticle(d) => &d.publication_title,
            _ => return None,
        })
    }

    pub fn publisher(&self) -> Option<&str> {
        Some(match &self.data {
            ItemType::Book(d) => &d.publisher,
            ItemType::BookSection(d) => &d.publisher,
            ItemType::ConferencePaper(d) => &d.publisher,
            ItemType::DictionaryEntry(d) => &d.publisher,
            ItemType::Document(d) => &d.publisher,
            ItemType::EncyclopediaArticle(d) => &d.publisher,
            ItemType::Hearing(d) => &d.publisher,
            ItemType::Map(d) => &d.publisher,
            _ => return None,
        })
    }

    pub fn place(&self) -> Option<&str> {
        Some(match &self.data {
            ItemType::AudioRecording(d) => &d.place,
            ItemType::Book(d) => &d.place,
            ItemType::BookSection(d) => &d.place,
            ItemType::ComputerProgram(d) => &d.place,
            ItemType::ConferencePaper(d) => &d.place,
            ItemType::DictionaryEntry(d) => &d.place,
            ItemType::EncyclopediaArticle(d) => &d.place,
            ItemType::Hearing(d) => &d.place,
            ItemType::Manuscript(d) => &d.place,
            ItemType::Map(d) => &d.place,
            ItemType::NewspaperArticle(d) => &d.place,
            ItemType::Patent(d) => &d.place,
            ItemType::Presentation(d) => &d.place,
            ItemType::RadioBroadcast(d) => &d.place,
            ItemType::Report(d) => &d.place,
            ItemType::Thesis(d) => &d.place,
            ItemType::TvBroadcast(d) => &d.place,
            ItemType::VideoRecording(d) => &d.place,
            _ => return None,
        })
    }

    pub fn volume(&self) -> Option<&str> {
        Some(match &self.data {
            ItemType::AudioRecording(d) => &d.volume,
            ItemType::Book(d) => &d.volume,
            ItemType::BookSection(d) => &d.volume,
            ItemType::ConferencePaper(d) => &d.volume,
            ItemType::DictionaryEntry(d) => &d.volume,
            ItemType::EncyclopediaArticle(d) => &d.volume,
            ItemType::JournalArticle(d) => &d.volume,
            ItemType::MagazineArticle(d) => &d.volume,
            ItemType::VideoRecording(d) => &d.volume,
            _ => return None,
        })
    }

    pub fn issue(&self) -> Option<&str> {
        Some(match &self.data {
            ItemType::JournalArticle(d) => &d.issue,
            ItemType::MagazineArticle(d) => &d.issue,
            _ => return None,
        })
    }

    pub fn section(&self) -> Option<&str> {
        Some(match &self.data {
            ItemType::Bill(d) => &d.section,
            ItemType::NewspaperArticle(d) => &d.section,
            ItemType::Statute(d) => &d.section,
            _ => return None,
        })
    }

    /// THe `part_number` from the underlying `data`.
    ///
    /// **TODO**: This is not actually wired up in any of the [ItemType]s
    /// as far as I can tell, so this needs to be done too.
    pub fn part_number(&self) -> Option<&str> {
        None
    }

    /// THe `part_number` from the underlying `data`.
    ///
    /// **TODO**: This is not actually wired up in any of the [ItemType]s
    /// as far as I can tell, so this needs to be done too.
    pub fn part_title(&self) -> Option<&str> {
        None
    }

    pub fn pages(&self) -> Option<&str> {
        Some(match &self.data {
            ItemType::BookSection(d) => &d.pages,
            ItemType::ConferencePaper(d) => &d.pages,
            ItemType::DictionaryEntry(d) => &d.pages,
            ItemType::EncyclopediaArticle(d) => &d.pages,
            ItemType::Hearing(d) => &d.pages,
            ItemType::JournalArticle(d) => &d.pages,
            ItemType::MagazineArticle(d) => &d.pages,
            ItemType::NewspaperArticle(d) => &d.pages,
            ItemType::Patent(d) => &d.pages,
            ItemType::Report(d) => &d.pages,
            ItemType::Statute(d) => &d.pages,
            _ => return None,
        })
    }

    pub fn series(&self) -> Option<&str> {
        Some(match &self.data {
            ItemType::Book(d) => &d.series,
            ItemType::BookSection(d) => &d.series,
            ItemType::ConferencePaper(d) => &d.series,
            ItemType::DictionaryEntry(d) => &d.series,
            ItemType::EncyclopediaArticle(d) => &d.series,
            ItemType::JournalArticle(d) => &d.series,
            _ => return None,
        })
    }

    pub fn series_title(&self) -> Option<&str> {
        Some(
            match &self.data {
                ItemType::AudioRecording(d) => &d.series_title,
                ItemType::ComputerProgram(d) => &d.series_title,
                ItemType::JournalArticle(d) => &d.series_title,
                ItemType::Map(d) => &d.series_title,
                ItemType::Podcast(d) => &d.series_title,
                ItemType::Report(d) => &d.series_title,
                ItemType::VideoRecording(d) => &d.series_title,
                _ => return None,
            }
        )
    }

    /// The `series_text` from the underlying `data`.
    ///
    /// **TODO**: This is not actually wired up in any of the [ItemType]s
    /// as far as I can tell, so this needs to be done too.
    pub fn series_text(&self) -> Option<&str> {
        None
    }

    /// The `journal_abbreviation` from the underlying `data`.
    ///
    /// **TODO**: This is not actually wired up in any of the [ItemType]s
    /// as far as I can tell, so this needs to be done too.
    pub fn journal_abbreviation(&self) -> Option<&str> {
        None
    }

    /// The `citation_key` from the underlying `data`.
    ///
    /// **TODO**: This is not actually wired up in any of the [ItemType]s
    /// as far as I can tell, so this needs to be done too.
    pub fn citation_key(&self) -> Option<&str> {
        None
    }

    pub fn url(&self) -> Option<&str> {
        Some(
            match &self.data {
                ItemType::Artwork(d) => &d.url,
                ItemType::AudioRecording(d) => &d.url,
                ItemType::Bill(d) => &d.url,
                ItemType::BlogPost(d) => &d.url,
                ItemType::Book(d) => &d.url,
                ItemType::BookSection(d) => &d.url,
                ItemType::Case(d) => &d.url,
                ItemType::ComputerProgram(d) => &d.url,
                ItemType::ConferencePaper(d) => &d.url,
                ItemType::DictionaryEntry(d) => &d.url,
                ItemType::Document(d) => &d.url,
                ItemType::Email(d) => &d.url,
                ItemType::EncyclopediaArticle(d) => &d.url,
                ItemType::Film(d) => &d.url,
                ItemType::ForumPost(d) => &d.url,
                ItemType::Hearing(d) => &d.url,
                ItemType::InstantMessage(d) => &d.url,
                ItemType::Interview(d) => &d.url,
                ItemType::JournalArticle(d) => &d.url,
                ItemType::Letter(d) => &d.url,
                ItemType::MagazineArticle(d) => &d.url,
                ItemType::Manuscript(d) => &d.url,
                ItemType::Map(d) => &d.url,
                ItemType::NewspaperArticle(d) => &d.url,
                ItemType::Patent(d) => &d.url,
                ItemType::Podcast(d) => &d.url,
                ItemType::Presentation(d) => &d.url,
                ItemType::RadioBroadcast(d) => &d.url,
                ItemType::Report(d) => &d.url,
                ItemType::Statute(d) => &d.url,
                ItemType::Thesis(d) => &d.url,
                ItemType::TvBroadcast(d) => &d.url,
                ItemType::VideoRecording(d) => &d.url,
                ItemType::Webpage(d) => &d.url,
                ItemType::Attachment(d) => &d.url,
                _ => return None,
            }
        )
    }

    pub fn access_date(&self) -> Option<DateTime<Local>> {
        let date_str = match &self.data {
            ItemType::Artwork(d) => &d.access_date,
            ItemType::AudioRecording(d) => &d.access_date,
            ItemType::Bill(d) => &d.access_date,
            ItemType::BlogPost(d) => &d.access_date,
            ItemType::Book(d) => &d.access_date,
            ItemType::BookSection(d) => &d.access_date,
            ItemType::Case(d) => &d.access_date,
            ItemType::ComputerProgram(d) => &d.access_date,
            ItemType::ConferencePaper(d) => &d.access_date,
            ItemType::DictionaryEntry(d) => &d.access_date,
            ItemType::Document(d) => &d.access_date,
            ItemType::Email(d) => &d.access_date,
            ItemType::EncyclopediaArticle(d) => &d.access_date,
            ItemType::Film(d) => &d.access_date,
            ItemType::ForumPost(d) => &d.access_date,
            ItemType::Hearing(d) => &d.access_date,
            ItemType::InstantMessage(d) => &d.access_date,
            ItemType::Interview(d) => &d.access_date,
            ItemType::JournalArticle(d) => &d.access_date,
            ItemType::Letter(d) => &d.access_date,
            ItemType::MagazineArticle(d) => &d.access_date,
            ItemType::Manuscript(d) => &d.access_date,
            ItemType::Map(d) => &d.access_date,
            ItemType::NewspaperArticle(d) => &d.access_date,
            ItemType::Patent(d) => &d.access_date,
            ItemType::Podcast(d) => &d.access_date,
            ItemType::Presentation(d) => &d.access_date,
            ItemType::RadioBroadcast(d) => &d.access_date,
            ItemType::Report(d) => &d.access_date,
            ItemType::Statute(d) => &d.access_date,
            ItemType::Thesis(d) => &d.access_date,
            ItemType::TvBroadcast(d) => &d.access_date,
            ItemType::VideoRecording(d) => &d.access_date,
            ItemType::Webpage(d) => &d.access_date,
            ItemType::Attachment(d) => &d.access_date,
            _ => return None,
        };
        Some(convert_zotero_date_str(date_str))
    }

    /// The `PMID` from the underlying `data`.
    ///
    /// **TODO**: This is not actually wired up in any of the [ItemType]s
    /// as far as I can tell, so this needs to be done too.
    pub fn pmid(&self) -> Option<&str> {
        None
    }

    /// The `PMCID` from the underlying `data`.
    ///
    /// **TODO**: This is not actually wired up in any of the [ItemType]s
    /// as far as I can tell, so this needs to be done too.
    pub fn pmcid(&self) -> Option<&str> {
        None
    }

    pub fn issn(&self) -> Option<&str> {
        Some(
            match &self.data {
                ItemType::JournalArticle(d) => &d.issn,
                ItemType::MagazineArticle(d) => &d.issn,
                ItemType::NewspaperArticle(d) => &d.issn,
                _ => return None,
            }
        )
    }

    pub fn isbn(&self) -> Option<&str> {
        Some(
            match &self.data {
                ItemType::AudioRecording(d) => &d.isbn,
                ItemType::Book(d) => &d.isbn,
                ItemType::BookSection(d) => &d.isbn,
                ItemType::ComputerProgram(d) => &d.isbn,
                ItemType::ConferencePaper(d) => &d.isbn,
                ItemType::DictionaryEntry(d) => &d.isbn,
                ItemType::EncyclopediaArticle(d) => &d.isbn,
                ItemType::Map(d) => &d.isbn,
                ItemType::VideoRecording(d) => &d.isbn,
                _ => return None,
            }
        )
    }

    pub fn doi(&self) -> Option<&str> {
        Some(
            match &self.data {
                ItemType::ConferencePaper(d) => &d.doi,
                ItemType::JournalArticle(d) => &d.doi,
                _ => return None,
            }
        )
    }

    pub fn archive(&self) -> Option<&str> {
        Some(
            match &self.data {
                ItemType::Artwork(d) => &d.archive,
                ItemType::AudioRecording(d) => &d.archive,
                ItemType::Book(d) => &d.archive,
                ItemType::BookSection(d) => &d.archive,
                ItemType::ComputerProgram(d) => &d.archive,
                ItemType::ConferencePaper(d) => &d.archive,
                ItemType::DictionaryEntry(d) => &d.archive,
                ItemType::Document(d) => &d.archive,
                ItemType::EncyclopediaArticle(d) => &d.archive,
                ItemType::Film(d) => &d.archive,
                ItemType::Interview(d) => &d.archive,
                ItemType::JournalArticle(d) => &d.archive,
                ItemType::Letter(d) => &d.archive,
                ItemType::MagazineArticle(d) => &d.archive,
                ItemType::Manuscript(d) => &d.archive,
                ItemType::Map(d) => &d.archive,
                ItemType::NewspaperArticle(d) => &d.archive,
                ItemType::RadioBroadcast(d) => &d.archive,
                ItemType::Report(d) => &d.archive,
                ItemType::Thesis(d) => &d.archive,
                ItemType::TvBroadcast(d) => &d.archive,
                ItemType::VideoRecording(d) => &d.archive,
                _ => return None,
            }
        )
    }

    pub fn archive_location(&self) -> Option<&str> {
        Some(
            match &self.data {
                ItemType::Artwork(d) => &d.archive_location,
                ItemType::AudioRecording(d) => &d.archive_location,
                ItemType::Book(d) => &d.archive_location,
                ItemType::BookSection(d) => &d.archive_location,
                ItemType::ComputerProgram(d) => &d.archive_location,
                ItemType::ConferencePaper(d) => &d.archive_location,
                ItemType::DictionaryEntry(d) => &d.archive_location,
                ItemType::Document(d) => &d.archive_location,
                ItemType::EncyclopediaArticle(d) => &d.archive_location,
                ItemType::Film(d) => &d.archive_location,
                ItemType::Interview(d) => &d.archive_location,
                ItemType::JournalArticle(d) => &d.archive_location,
                ItemType::Letter(d) => &d.archive_location,
                ItemType::MagazineArticle(d) => &d.archive_location,
                ItemType::Manuscript(d) => &d.archive_location,
                ItemType::Map(d) => &d.archive_location,
                ItemType::NewspaperArticle(d) => &d.archive_location,
                ItemType::RadioBroadcast(d) => &d.archive_location,
                ItemType::Report(d) => &d.archive_location,
                ItemType::Thesis(d) => &d.archive_location,
                ItemType::TvBroadcast(d) => &d.archive_location,
                ItemType::VideoRecording(d) => &d.archive_location,
                _ => return None,
            }
        )
    }

    pub fn short_title(&self) -> Option<&str> {
        Some(
            match &self.data {
                ItemType::Artwork(d) => &d.short_title,
                ItemType::AudioRecording(d) => &d.short_title,
                ItemType::Bill(d) => &d.short_title,
                ItemType::BlogPost(d) => &d.short_title,
                ItemType::Book(d) => &d.short_title,
                ItemType::BookSection(d) => &d.short_title,
                ItemType::ComputerProgram(d) => &d.short_title,
                ItemType::ConferencePaper(d) => &d.short_title,
                ItemType::DictionaryEntry(d) => &d.short_title,
                ItemType::Document(d) => &d.short_title,
                ItemType::EncyclopediaArticle(d) => &d.short_title,
                ItemType::Film(d) => &d.short_title,
                ItemType::ForumPost(d) => &d.short_title,
                ItemType::Hearing(d) => &d.short_title,
                ItemType::InstantMessage(d) => &d.short_title,
                ItemType::Interview(d) => &d.short_title,
                ItemType::JournalArticle(d) => &d.short_title,
                ItemType::Letter(d) => &d.short_title,
                ItemType::MagazineArticle(d) => &d.short_title,
                ItemType::Manuscript(d) => &d.short_title,
                ItemType::Map(d) => &d.short_title,
                ItemType::NewspaperArticle(d) => &d.short_title,
                ItemType::Patent(d) => &d.short_title,
                ItemType::Podcast(d) => &d.short_title,
                ItemType::Presentation(d) => &d.short_title,
                ItemType::RadioBroadcast(d) => &d.short_title,
                ItemType::Report(d) => &d.short_title,
                ItemType::Thesis(d) => &d.short_title,
                ItemType::TvBroadcast(d) => &d.short_title,
                ItemType::VideoRecording(d) => &d.short_title,
                ItemType::Webpage(d) => &d.short_title,
                _ => return None,
            }
        )
    }

    pub fn language(&self) -> Option<&str> {
        Some(
            match &self.data {
                ItemType::Artwork(d) => &d.language,
                ItemType::AudioRecording(d) => &d.language,
                ItemType::Bill(d) => &d.language,
                ItemType::BlogPost(d) => &d.language,
                ItemType::Book(d) => &d.language,
                ItemType::BookSection(d) => &d.language,
                ItemType::Case(d) => &d.language,
                ItemType::ConferencePaper(d) => &d.language,
                ItemType::DictionaryEntry(d) => &d.language,
                ItemType::Document(d) => &d.language,
                ItemType::Email(d) => &d.language,
                ItemType::EncyclopediaArticle(d) => &d.language,
                ItemType::Film(d) => &d.language,
                ItemType::ForumPost(d) => &d.language,
                ItemType::Hearing(d) => &d.language,
                ItemType::InstantMessage(d) => &d.language,
                ItemType::Interview(d) => &d.language,
                ItemType::JournalArticle(d) => &d.language,
                ItemType::Letter(d) => &d.language,
                ItemType::MagazineArticle(d) => &d.language,
                ItemType::Manuscript(d) => &d.language,
                ItemType::Map(d) => &d.language,
                ItemType::NewspaperArticle(d) => &d.language,
                ItemType::Patent(d) => &d.language,
                ItemType::Podcast(d) => &d.language,
                ItemType::Presentation(d) => &d.language,
                ItemType::RadioBroadcast(d) => &d.language,
                ItemType::Report(d) => &d.language,
                ItemType::Statute(d) => &d.language,
                ItemType::Thesis(d) => &d.language,
                ItemType::TvBroadcast(d) => &d.language,
                ItemType::VideoRecording(d) => &d.language,
                ItemType::Webpage(d) => &d.language,
                _ => return None,
            }
        )
    }

    pub fn library_catalog(&self) -> Option<&str> {
        Some(
            match &self.data {
                ItemType::Artwork(d) => &d.library_catalog,
                ItemType::AudioRecording(d) => &d.library_catalog,
                ItemType::Book(d) => &d.library_catalog,
                ItemType::BookSection(d) => &d.library_catalog,
                ItemType::ComputerProgram(d) => &d.library_catalog,
                ItemType::ConferencePaper(d) => &d.library_catalog,
                ItemType::DictionaryEntry(d) => &d.library_catalog,
                ItemType::Document(d) => &d.library_catalog,
                ItemType::EncyclopediaArticle(d) => &d.library_catalog,
                ItemType::Film(d) => &d.library_catalog,
                ItemType::Interview(d) => &d.library_catalog,
                ItemType::JournalArticle(d) => &d.library_catalog,
                ItemType::Letter(d) => &d.library_catalog,
                ItemType::MagazineArticle(d) => &d.library_catalog,
                ItemType::Manuscript(d) => &d.library_catalog,
                ItemType::Map(d) => &d.library_catalog,
                ItemType::NewspaperArticle(d) => &d.library_catalog,
                ItemType::RadioBroadcast(d) => &d.library_catalog,
                ItemType::Report(d) => &d.library_catalog,
                ItemType::Thesis(d) => &d.library_catalog,
                ItemType::TvBroadcast(d) => &d.library_catalog,
                ItemType::VideoRecording(d) => &d.library_catalog,
                _ => return None,
            }
        )
    }

    pub fn call_number(&self) -> Option<&str> {
        Some(
            match &self.data {
                ItemType::Artwork(d) => &d.call_number,
                ItemType::AudioRecording(d) => &d.call_number,
                ItemType::Book(d) => &d.call_number,
                ItemType::BookSection(d) => &d.call_number,
                ItemType::ComputerProgram(d) => &d.call_number,
                ItemType::ConferencePaper(d) => &d.call_number,
                ItemType::DictionaryEntry(d) => &d.call_number,
                ItemType::Document(d) => &d.call_number,
                ItemType::EncyclopediaArticle(d) => &d.call_number,
                ItemType::Film(d) => &d.call_number,
                ItemType::Interview(d) => &d.call_number,
                ItemType::JournalArticle(d) => &d.call_number,
                ItemType::Letter(d) => &d.call_number,
                ItemType::MagazineArticle(d) => &d.call_number,
                ItemType::Manuscript(d) => &d.call_number,
                ItemType::Map(d) => &d.call_number,
                ItemType::NewspaperArticle(d) => &d.call_number,
                ItemType::RadioBroadcast(d) => &d.call_number,
                ItemType::Report(d) => &d.call_number,
                ItemType::Thesis(d) => &d.call_number,
                ItemType::TvBroadcast(d) => &d.call_number,
                ItemType::VideoRecording(d) => &d.call_number,
                _ => return None,
            }
        )
    }

    pub fn rights(&self) -> Option<&str> {
        Some(
            match &self.data {
                ItemType::Artwork(d) => &d.rights,
                ItemType::AudioRecording(d) => &d.rights,
                ItemType::Bill(d) => &d.rights,
                ItemType::BlogPost(d) => &d.rights,
                ItemType::Book(d) => &d.rights,
                ItemType::BookSection(d) => &d.rights,
                ItemType::Case(d) => &d.rights,
                ItemType::ComputerProgram(d) => &d.rights,
                ItemType::ConferencePaper(d) => &d.rights,
                ItemType::DictionaryEntry(d) => &d.rights,
                ItemType::Document(d) => &d.rights,
                ItemType::Email(d) => &d.rights,
                ItemType::EncyclopediaArticle(d) => &d.rights,
                ItemType::Film(d) => &d.rights,
                ItemType::ForumPost(d) => &d.rights,
                ItemType::Hearing(d) => &d.rights,
                ItemType::InstantMessage(d) => &d.rights,
                ItemType::Interview(d) => &d.rights,
                ItemType::JournalArticle(d) => &d.rights,
                ItemType::Letter(d) => &d.rights,
                ItemType::MagazineArticle(d) => &d.rights,
                ItemType::Manuscript(d) => &d.rights,
                ItemType::Map(d) => &d.rights,
                ItemType::NewspaperArticle(d) => &d.rights,
                ItemType::Patent(d) => &d.rights,
                ItemType::Podcast(d) => &d.rights,
                ItemType::Presentation(d) => &d.rights,
                ItemType::RadioBroadcast(d) => &d.rights,
                ItemType::Report(d) => &d.rights,
                ItemType::Statute(d) => &d.rights,
                ItemType::Thesis(d) => &d.rights,
                ItemType::TvBroadcast(d) => &d.rights,
                ItemType::VideoRecording(d) => &d.rights,
                ItemType::Webpage(d) => &d.rights,
                _ => return None,
            }
        )
    }

    pub fn extra(&self) -> Option<&str> {
        Some(
            match &self.data {
                ItemType::Artwork(d) => &d.extra,
                ItemType::AudioRecording(d) => &d.extra,
                ItemType::Bill(d) => &d.extra,
                ItemType::BlogPost(d) => &d.extra,
                ItemType::Book(d) => &d.extra,
                ItemType::BookSection(d) => &d.extra,
                ItemType::Case(d) => &d.extra,
                ItemType::ComputerProgram(d) => &d.extra,
                ItemType::ConferencePaper(d) => &d.extra,
                ItemType::DictionaryEntry(d) => &d.extra,
                ItemType::Document(d) => &d.extra,
                ItemType::Email(d) => &d.extra,
                ItemType::EncyclopediaArticle(d) => &d.extra,
                ItemType::Film(d) => &d.extra,
                ItemType::ForumPost(d) => &d.extra,
                ItemType::Hearing(d) => &d.extra,
                ItemType::InstantMessage(d) => &d.extra,
                ItemType::Interview(d) => &d.extra,
                ItemType::JournalArticle(d) => &d.extra,
                ItemType::Letter(d) => &d.extra,
                ItemType::MagazineArticle(d) => &d.extra,
                ItemType::Manuscript(d) => &d.extra,
                ItemType::Map(d) => &d.extra,
                ItemType::NewspaperArticle(d) => &d.extra,
                ItemType::Patent(d) => &d.extra,
                ItemType::Podcast(d) => &d.extra,
                ItemType::Presentation(d) => &d.extra,
                ItemType::RadioBroadcast(d) => &d.extra,
                ItemType::Report(d) => &d.extra,
                ItemType::Statute(d) => &d.extra,
                ItemType::Thesis(d) => &d.extra,
                ItemType::TvBroadcast(d) => &d.extra,
                ItemType::VideoRecording(d) => &d.extra,
                ItemType::Webpage(d) => &d.extra,
                _ => return None,
            }
        )
    }

    pub fn note(&self) -> Option<&str> {
        Some(match &self.data {
            ItemType::Attachment(d) => &d.note,
            ItemType::Note(d) => &d.note,
            _ => return None,
        })
    }

    pub fn date_added(&self) -> DateTime<Local> {
        let date_str = match &self.data {
            ItemType::Artwork(d) => &d.date_added,
            ItemType::AudioRecording(d) => &d.date_added,
            ItemType::Bill(d) => &d.date_added,
            ItemType::BlogPost(d) => &d.date_added,
            ItemType::Book(d) => &d.date_added,
            ItemType::BookSection(d) => &d.date_added,
            ItemType::Case(d) => &d.date_added,
            ItemType::ComputerProgram(d) => &d.date_added,
            ItemType::ConferencePaper(d) => &d.date_added,
            ItemType::DictionaryEntry(d) => &d.date_added,
            ItemType::Document(d) => &d.date_added,
            ItemType::Email(d) => &d.date_added,
            ItemType::EncyclopediaArticle(d) => &d.date_added,
            ItemType::Film(d) => &d.date_added,
            ItemType::ForumPost(d) => &d.date_added,
            ItemType::Hearing(d) => &d.date_added,
            ItemType::InstantMessage(d) => &d.date_added,
            ItemType::Interview(d) => &d.date_added,
            ItemType::JournalArticle(d) => &d.date_added,
            ItemType::Letter(d) => &d.date_added,
            ItemType::MagazineArticle(d) => &d.date_added,
            ItemType::Manuscript(d) => &d.date_added,
            ItemType::Map(d) => &d.date_added,
            ItemType::NewspaperArticle(d) => &d.date_added,
            ItemType::Patent(d) => &d.date_added,
            ItemType::Podcast(d) => &d.date_added,
            ItemType::Presentation(d) => &d.date_added,
            ItemType::RadioBroadcast(d) => &d.date_added,
            ItemType::Report(d) => &d.date_added,
            ItemType::Statute(d) => &d.date_added,
            ItemType::Thesis(d) => &d.date_added,
            ItemType::TvBroadcast(d) => &d.date_added,
            ItemType::VideoRecording(d) => &d.date_added,
            ItemType::Webpage(d) => &d.date_added,
            ItemType::Attachment(d) => &d.date_added,
            ItemType::Note(d) => &d.date_added,
        };
        convert_zotero_date_str(date_str)
    }

    pub fn date_modified(&self) -> Option<DateTime<Local>> {
        let date_str = match &self.data {
            ItemType::Artwork(d) => &d.date_modified,
            ItemType::AudioRecording(d) => &d.date_modified,
            ItemType::Bill(d) => &d.date_modified,
            ItemType::BlogPost(d) => &d.date_modified,
            ItemType::Book(d) => &d.date_modified,
            ItemType::BookSection(d) => &d.date_modified,
            ItemType::Case(d) => &d.date_modified,
            ItemType::ComputerProgram(d) => &d.date_modified,
            ItemType::ConferencePaper(d) => &d.date_modified,
            ItemType::DictionaryEntry(d) => &d.date_modified,
            ItemType::Document(d) => &d.date_modified,
            ItemType::Email(d) => &d.date_modified,
            ItemType::EncyclopediaArticle(d) => &d.date_modified,
            ItemType::Film(d) => &d.date_modified,
            ItemType::ForumPost(d) => &d.date_modified,
            ItemType::Hearing(d) => &d.date_modified,
            ItemType::InstantMessage(d) => &d.date_modified,
            ItemType::Interview(d) => &d.date_modified,
            ItemType::JournalArticle(d) => &d.date_modified,
            ItemType::Letter(d) => &d.date_modified,
            ItemType::MagazineArticle(d) => &d.date_modified,
            ItemType::Manuscript(d) => &d.date_modified,
            ItemType::Map(d) => &d.date_modified,
            ItemType::NewspaperArticle(d) => &d.date_modified,
            ItemType::Patent(d) => &d.date_modified,
            ItemType::Podcast(d) => &d.date_modified,
            ItemType::Presentation(d) => &d.date_modified,
            ItemType::RadioBroadcast(d) => &d.date_modified,
            ItemType::Report(d) => &d.date_modified,
            ItemType::Statute(d) => &d.date_modified,
            ItemType::Thesis(d) => &d.date_modified,
            ItemType::TvBroadcast(d) => &d.date_modified,
            ItemType::VideoRecording(d) => &d.date_modified,
            ItemType::Webpage(d) => &d.date_modified,
            ItemType::Note(d) => &d.date_modified,
            // TODO: Odd inconsistency in the underlying.
            ItemType::Attachment(d) => {
                // special acrobatics
                return d.date_modified.as_ref().map(convert_zotero_date_str);
            }
        };
        Some(convert_zotero_date_str(date_str))
    }
}

static DATE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap());

static FORMATTER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d{2})/(\d{4})").unwrap());

fn convert_zotero_date_str<D: AsRef<str>>(date_str: D) -> DateTime<Local> {
    let date_str = date_str.as_ref();
    let date_captures = DATE_REGEX.captures(date_str);

    let formatter_captures = FORMATTER_REGEX.captures(date_str);

    let expanded_date = if let Ok(d) = DateTime::parse_from_rfc3339(date_str) {
        d.naive_local()
    } else if let Some(captures) = date_captures {
        // Date is in the "YYYY-MM-DD" format
        let year = captures[1].parse::<i32>().unwrap();
        let month = captures[2].parse::<u32>().unwrap();
        let day = captures[3].parse::<u32>().unwrap();
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(year, month, day).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        )
    } else if let Some(captures) = formatter_captures {
        // Date is in the "MM/YYYY" format
        let month = captures[1].parse::<u32>().unwrap();
        let year = captures[2].parse::<i32>().unwrap();
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(year, month, 1).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        )
    } else {
        // Unrecognized date format
        return Local::now();
    };

    // Convert the NaiveDateTime object to a DateTime object in the local timezone
    expanded_date.and_local_timezone(Local).unwrap()
}

#[derive(Deserialize, Serialize, Default, Clone, Debug, Builder, PartialEq)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct Creator {
    #[serde(alias = "creatorType")]
    pub creator_type: String,
    #[serde(alias = "firstName")]
    pub first_name: String,
    #[serde(alias = "lastName")]
    pub last_name: String,
}

impl Creator {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn short_name(&self) -> String {
        match self.first_name.chars().next() {
            Some(first_initial) => format!("{}. {}", first_initial, self.last_name),
            None => self.last_name.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct ItemMeta {
    pub creator_summary: Option<String>,
    pub parsed_date: Option<String>,
    #[serde(default)]
    pub num_children: Option<SizeOrBool>,
    // The following part concerns collections
    pub num_collections: Option<usize>,
    pub num_items: Option<usize>,
}

impl ItemMeta {
    pub fn has_children(&self) -> bool {
        match &self.num_children {
            None => false,
            Some(sob) => match sob {
                SizeOrBool::Bool(_) => false,
                SizeOrBool::Size(v) => *v > 0,
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SizeOrBool {
    Bool(bool),
    Size(usize),
}

impl Default for SizeOrBool {
    fn default() -> SizeOrBool {
        SizeOrBool::Bool(false)
    }
}

#[cfg(test)]
mod test_item_deserialization {
    use super::*;
    #[test]
    fn item_deserialization() {
        let input = r#"
            {
                "key": "4X5CQGQA",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/4X5CQGQA",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/4X5CQGQA",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "4X5CQGQA",
                    "version": 2444,
                    "itemType": "bookSection",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "bookTitle": "",
                    "series": "",
                    "seriesNumber": "",
                    "volume": "",
                    "numberOfVolumes": "",
                    "edition": "",
                    "place": "",
                    "publisher": "",
                    "date": "",
                    "pages": "",
                    "language": "",
                    "ISBN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:17:23Z",
                    "dateModified": "2019-10-01T21:17:23Z"
                }
            }
        "#;

        serde_json::from_str::<Item>(input).expect("Failed to parse zotero data");
        assert!(true)
    }
    #[test]
    fn items_deserialization() {
        let input = r#"
        [
            {
                "key": "4X5CQGQA",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/4X5CQGQA",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/4X5CQGQA",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "4X5CQGQA",
                    "version": 2444,
                    "itemType": "bookSection",
                    "title": "",
                    "creators": [
                        {
                            "creatorType": "author",
                            "firstName": "John",
                            "lastName": "Doe"
                        }
                    ],
                    "abstractNote": "",
                    "bookTitle": "",
                    "series": "",
                    "seriesNumber": "",
                    "volume": "",
                    "numberOfVolumes": "",
                    "edition": "",
                    "place": "",
                    "publisher": "",
                    "date": "",
                    "pages": "",
                    "language": "",
                    "ISBN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:17:23Z",
                    "dateModified": "2019-10-01T21:17:23Z"
                }
            },
            {
                "key": "4H5SBMDE",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/4H5SBMDE",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/4H5SBMDE",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": false
                },
                "data": {
                    "key": "4H5SBMDE",
                    "version": 2444,
                    "itemType": "map",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "mapType": "",
                    "scale": "",
                    "seriesTitle": "",
                    "edition": "",
                    "place": "",
                    "publisher": "",
                    "date": "",
                    "language": "",
                    "ISBN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:17:19Z",
                    "dateModified": "2019-10-01T21:17:19Z"
                }
            },
            {
                "key": "SAU7PP79",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/SAU7PP79",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/SAU7PP79",
                        "type": "text/html"
                    }
                },
                "meta": {
                },
                "data": {
                    "key": "SAU7PP79",
                    "version": 2444,
                    "itemType": "patent",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "place": "",
                    "country": "",
                    "assignee": "",
                    "issuingAuthority": "",
                    "patentNumber": "",
                    "filingDate": "",
                    "pages": "",
                    "applicationNumber": "",
                    "priorityNumbers": "",
                    "issueDate": "",
                    "references": "",
                    "legalStatus": "",
                    "language": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:17:16Z",
                    "dateModified": "2019-10-01T21:17:16Z"
                }
            },
            {
                "key": "25DYFG56",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/25DYFG56",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/25DYFG56",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "25DYFG56",
                    "version": 2444,
                    "itemType": "blogPost",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "blogTitle": "",
                    "websiteType": "",
                    "date": "",
                    "url": "",
                    "accessDate": "",
                    "language": "",
                    "shortTitle": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:17:13Z",
                    "dateModified": "2019-10-01T21:17:13Z"
                }
            },
            {
                "key": "U4AP5MUH",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/U4AP5MUH",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/U4AP5MUH",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "U4AP5MUH",
                    "version": 2444,
                    "itemType": "podcast",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "seriesTitle": "",
                    "episodeNumber": "",
                    "audioFileType": "",
                    "runningTime": "",
                    "url": "",
                    "accessDate": "",
                    "language": "",
                    "shortTitle": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:17:10Z",
                    "dateModified": "2019-10-01T21:17:10Z"
                }
            },
            {
                "key": "SD5EWBBC",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/SD5EWBBC",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/SD5EWBBC",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "SD5EWBBC",
                    "version": 2444,
                    "itemType": "journalArticle",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "publicationTitle": "",
                    "volume": "",
                    "issue": "",
                    "pages": "",
                    "date": "",
                    "series": "",
                    "seriesTitle": "",
                    "seriesText": "",
                    "journalAbbreviation": "",
                    "language": "",
                    "DOI": "",
                    "ISSN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:16:58Z",
                    "dateModified": "2019-10-01T21:16:58Z"
                }
            },
            {
                "key": "9XZ2W4RU",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/9XZ2W4RU",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/9XZ2W4RU",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "9XZ2W4RU",
                    "version": 2444,
                    "itemType": "magazineArticle",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "publicationTitle": "",
                    "volume": "",
                    "issue": "",
                    "date": "",
                    "pages": "",
                    "language": "",
                    "ISSN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:16:56Z",
                    "dateModified": "2019-10-01T21:16:56Z"
                }
            },
            {
                "key": "EFZTBRBT",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/EFZTBRBT",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/EFZTBRBT",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "EFZTBRBT",
                    "version": 2444,
                    "itemType": "newspaperArticle",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "publicationTitle": "",
                    "place": "",
                    "edition": "",
                    "date": "",
                    "section": "",
                    "pages": "",
                    "language": "",
                    "shortTitle": "",
                    "ISSN": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:16:53Z",
                    "dateModified": "2019-10-01T21:16:53Z"
                }
            },
            {
                "key": "5G7FQMJH",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/5G7FQMJH",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/5G7FQMJH",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "5G7FQMJH",
                    "version": 2444,
                    "itemType": "conferencePaper",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "date": "",
                    "proceedingsTitle": "",
                    "conferenceName": "",
                    "place": "",
                    "publisher": "",
                    "volume": "",
                    "pages": "",
                    "series": "",
                    "language": "",
                    "DOI": "",
                    "ISBN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:16:50Z",
                    "dateModified": "2019-10-01T21:16:50Z"
                }
            },
            {
                "key": "48UAQNNN",
                "version": 2444,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/48UAQNNN",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/48UAQNNN",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "48UAQNNN",
                    "version": 2444,
                    "itemType": "encyclopediaArticle",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "encyclopediaTitle": "",
                    "series": "",
                    "seriesNumber": "",
                    "volume": "",
                    "numberOfVolumes": "",
                    "edition": "",
                    "place": "",
                    "publisher": "",
                    "date": "",
                    "pages": "",
                    "ISBN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "language": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T21:16:45Z",
                    "dateModified": "2019-10-01T21:16:45Z"
                }
            },
            {
                "key": "5I47RLH3",
                "version": 2438,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/5I47RLH3",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/5I47RLH3",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "5I47RLH3",
                    "version": 2438,
                    "itemType": "videoRecording",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "videoRecordingFormat": "",
                    "seriesTitle": "",
                    "volume": "",
                    "numberOfVolumes": "",
                    "place": "",
                    "studio": "",
                    "date": "",
                    "runningTime": "",
                    "language": "",
                    "ISBN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:36:01Z",
                    "dateModified": "2019-10-01T13:36:01Z"
                }
            },
            {
                "key": "T9GKB3IR",
                "version": 2438,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/T9GKB3IR",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/T9GKB3IR",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "T9GKB3IR",
                    "version": 2438,
                    "itemType": "radioBroadcast",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "programTitle": "",
                    "episodeNumber": "",
                    "audioRecordingFormat": "",
                    "place": "",
                    "network": "",
                    "date": "",
                    "runningTime": "",
                    "language": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:35:16Z",
                    "dateModified": "2019-10-01T13:35:16Z"
                }
            },
            {
                "key": "QPDKPRGL",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/QPDKPRGL",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/QPDKPRGL",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "QPDKPRGL",
                    "version": 2437,
                    "itemType": "patent",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "place": "",
                    "country": "",
                    "assignee": "",
                    "issuingAuthority": "",
                    "patentNumber": "",
                    "filingDate": "",
                    "pages": "",
                    "applicationNumber": "",
                    "priorityNumbers": "",
                    "issueDate": "",
                    "references": "",
                    "legalStatus": "",
                    "language": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:35:07Z",
                    "dateModified": "2019-10-01T13:35:07Z"
                }
            },
            {
                "key": "MD62TVCE",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/MD62TVCE",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/MD62TVCE",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "MD62TVCE",
                    "version": 2437,
                    "itemType": "map",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "mapType": "",
                    "scale": "",
                    "seriesTitle": "",
                    "edition": "",
                    "place": "",
                    "publisher": "",
                    "date": "",
                    "language": "",
                    "ISBN": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "archive": "",
                    "archiveLocation": "",
                    "libraryCatalog": "",
                    "callNumber": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:35:01Z",
                    "dateModified": "2019-10-01T13:35:01Z"
                }
            },
            {
                "key": "L77RMQ8J",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/L77RMQ8J",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/L77RMQ8J",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "L77RMQ8J",
                    "version": 2437,
                    "itemType": "instantMessage",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "date": "",
                    "language": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:34:56Z",
                    "dateModified": "2019-10-01T13:34:56Z"
                }
            },
            {
                "key": "77NPSQY5",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/77NPSQY5",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/77NPSQY5",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "77NPSQY5",
                    "version": 2437,
                    "itemType": "hearing",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "committee": "",
                    "place": "",
                    "publisher": "",
                    "numberOfVolumes": "",
                    "documentNumber": "",
                    "pages": "",
                    "legislativeBody": "",
                    "session": "",
                    "history": "",
                    "date": "",
                    "language": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:34:50Z",
                    "dateModified": "2019-10-01T13:34:50Z"
                }
            },
            {
                "key": "3L59IWHK",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/3L59IWHK",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/3L59IWHK",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "3L59IWHK",
                    "version": 2437,
                    "itemType": "statute",
                    "nameOfAct": "",
                    "creators": [],
                    "abstractNote": "",
                    "code": "",
                    "codeNumber": "",
                    "publicLawNumber": "",
                    "dateEnacted": "",
                    "pages": "",
                    "section": "",
                    "session": "",
                    "history": "",
                    "language": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:34:43Z",
                    "dateModified": "2019-10-01T13:34:43Z"
                }
            },
            {
                "key": "QFRII6XJ",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/0000000/items/QFRII6XJ",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/QFRII6XJ",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "QFRII6XJ",
                    "version": 2437,
                    "itemType": "case",
                    "caseName": "",
                    "creators": [],
                    "abstractNote": "",
                    "reporter": "",
                    "reporterVolume": "",
                    "court": "",
                    "docketNumber": "",
                    "firstPage": "",
                    "history": "",
                    "dateDecided": "",
                    "language": "",
                    "shortTitle": "",
                    "url": "",
                    "accessDate": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:34:40Z",
                    "dateModified": "2019-10-01T13:34:40Z"
                }
            },
            {
                "key": "K36LXQI5",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/1000000/items/K36LXQI5",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/K36LXQI5",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "K36LXQI5",
                    "version": 2437,
                    "itemType": "bill",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "billNumber": "",
                    "code": "",
                    "codeVolume": "",
                    "section": "",
                    "codePages": "",
                    "legislativeBody": "",
                    "session": "",
                    "history": "",
                    "date": "",
                    "language": "",
                    "url": "",
                    "accessDate": "",
                    "shortTitle": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:34:30Z",
                    "dateModified": "2019-10-01T13:34:30Z"
                }
            }
        ]
        "#;
        serde_json::from_str::<Vec<Item>>(input).expect("Failed to parse zotero data");
        assert!(true);
    }

    #[test]
    fn test_deserialize_creators() {
        let expected_output = Creator {
            creator_type: "author".into(),
            first_name: "John".into(),
            last_name: "Doe".into(),
        };

        let input = r#"
        {
            "creatorType": "author",
            "firstName": "John",
            "lastName": "Doe"
        }
        "#;

        let result = serde_json::from_str::<Creator>(input).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_item_meta_deserialization() {
        let expected_output = ItemMeta {
            creator_summary: Some("Lorem".into()),
            parsed_date: Some("25-2-2019".into()),
            num_children: Some(SizeOrBool::Size(1)),
            num_collections: Some(1_usize),
            num_items: Some(0_usize),
        };

        let input = r#"
            {
                "creatorSummary" : "Lorem",
                "parsedDate": "25-2-2019",
                "numChildren": 1,
                "numCollections": 1,
                "numItems": 0
            }
        "#;

        let result = serde_json::from_str::<ItemMeta>(input).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_item_deserialization() {
        let input = r#"
            {
                "key": "K36LXQI5",
                "version": 2437,
                "library": {
                    "type": "user",
                    "id": 1000000,
                    "name": "john.doe",
                    "links": {
                        "alternate": {
                            "href": "https://www.zotero.org/john.doe",
                            "type": "text/html"
                        }
                    }
                },
                "links": {
                    "self": {
                        "href": "https://api.zotero.org/users/1000000/items/K36LXQI5",
                        "type": "application/json"
                    },
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe/items/K36LXQI5",
                        "type": "text/html"
                    }
                },
                "meta": {
                    "numChildren": 0
                },
                "data": {
                    "key": "K36LXQI5",
                    "version": 2437,
                    "itemType": "bill",
                    "title": "",
                    "creators": [],
                    "abstractNote": "",
                    "billNumber": "",
                    "code": "",
                    "codeVolume": "",
                    "section": "",
                    "codePages": "",
                    "legislativeBody": "",
                    "session": "",
                    "history": "",
                    "date": "",
                    "language": "",
                    "url": "",
                    "accessDate": "",
                    "shortTitle": "",
                    "rights": "",
                    "extra": "",
                    "tags": [],
                    "collections": [
                        "TYPDZEZF"
                    ],
                    "relations": {},
                    "dateAdded": "2019-10-01T13:34:30Z",
                    "dateModified": "2019-10-01T13:34:30Z"
                }
            }
        "#;

        serde_json::from_str::<Item>(input).expect("Failed to deserialize item");
        assert!(true);
    }
}
