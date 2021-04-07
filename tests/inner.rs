use lofty::VorbisTag;
use lofty::*;

#[test]
#[cfg(all(feature = "mp3", feature = "vorbis"))]
fn test_inner() {
	// New flac tag
	let mut innertag = metaflac::Tag::new();

	// Set the title of the flac tag
	innertag
		.vorbis_comments_mut()
		.set_title(vec!["title from metaflac::Tag"]);

	// Turn the flac tag into a VorbisTag
	let tag: VorbisTag = innertag.into();

	// Turn the VorbisTag into a Box<dyn AudioTag>
	let mut id3tag = tag.to_dyn_tag(TagType::Id3v2);

	// Write Box<dyn AudioTag> to `a.mp3`
	id3tag
		.write_to_path("tests/assets/a.mp3")
		.expect("Fail to write!");

	// Read from `a.mp3`
	let id3tag_reload = Tag::default()
		.read_from_path("tests/assets/a.mp3")
		.expect("Fail to read!");

	// Confirm title still matches
	assert_eq!(id3tag_reload.title(), Some("title from metaflac::Tag"));

	// Convert Box<dyn AudioTag> to id3::Tag
	let mut id3tag_inner: id3::Tag = id3tag_reload.into();

	// Create timestamp and change date_recorded
	let timestamp = id3::Timestamp {
		year: 2013,
		month: Some(2u8),
		day: Some(5u8),
		hour: Some(6u8),
		minute: None,
		second: None,
	};
	id3tag_inner.set_date_recorded(timestamp.clone());

	// Write id3::Tag to `a.mp3`
	id3tag_inner
		.write_to_path("tests/assets/a.mp3", id3::Version::Id3v24)
		.expect("Fail to write!");

	// Read from `a.mp3`
	let id3tag_reload = id3::Tag::read_from_path("tests/assets/a.mp3").expect("Fail to read!");

	// Confirm timestamp still matches
	assert_eq!(id3tag_reload.date_recorded(), Some(timestamp));
}
