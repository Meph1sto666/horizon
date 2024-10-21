use std::{fs::File, io::BufReader};
use ratatui::{text::Text, widgets::ListState};
use rodio::Decoder;
use symphonia::core::{formats::FormatOptions, io::{MediaSourceStream, MediaSourceStreamOptions}, meta::{Limit, MetadataOptions}};
use symphonia::default::get_probe;


pub struct Song {
    pub path: String,
	title: String,
	artist: String,
    album: String,
    track_num: String,
    album_tracks_total: String,
    year: String,
    // source: File,
    // stream: MediaSourceStream
}

impl Song {
    pub fn new(path: String) -> Self {
        let file: File = File::open(&path).expect("Failed to open file");
        let probe = get_probe();
        let mut format = probe.format(
            &Default::default(),
            MediaSourceStream::new(
                Box::new(file),
                MediaSourceStreamOptions { buffer_len: (1024*512) }
            ),
            &FormatOptions { prebuild_seek_index: true, seek_index_fill_rate: 128, enable_gapless: true },
            &MetadataOptions {
                limit_metadata_bytes: Limit::None,
                limit_visual_bytes: Limit::None
            }
        ).expect("Failed to probe format");
        

        let dta = format.metadata.get().unwrap();        
        let tags = dta.current().unwrap().tags();
        // println!();
        Song {
            path: path.clone(),
            title: tags[0].value.to_string(),
            artist: tags[1].value.to_string(),
            album: tags[4].value.to_string(),
            track_num: tags[2].value.to_string(),
            album_tracks_total: tags[3].value.to_string(),
            year: (tags[5].value.to_string()),
            // "TITLE:{}, ARTIST:{}, ALBUM:{}, TRACK_NUM:{}, TRACK_TOTAL:{}, YEAR:{}", tags[0].value, tags[1].value, tags[4].value, tags[2].value, tags[3].value, tags[5].value

            // source: std::fs::File::open(&path).expect("failed to open media"),
            // stream: MediaSourceStream::new(Box::new(std::fs::File::open(&path).expect("failed to open media")), Default::default()),
            // source: Decoder::new(BufReader::new(File::open(path).unwrap())).unwrap()
        }
	}
    pub fn get_source(&self) -> rodio::Decoder<std::io::BufReader<File>>{
        return Decoder::new(BufReader::new(File::open(self.path.clone()).unwrap())).unwrap()
    }
}

impl Clone for Song {
    fn clone(&self) -> Self {
        return Song {
            path: self.path.clone(),
            title:self.title.clone(),
            artist:self.artist.clone(),
            album:self.album.clone(),
            track_num:self.track_num.clone(),
            album_tracks_total:self.album_tracks_total.clone(),
            year: self.year.clone(),
            // source: self.source.try_clone().unwrap(),
            // stream: self.stream
        }
    }
}

// impl Copy for Song {
//     fn Copy(value: Song) -> Self {
//         return Song {value: value.}
//     }
// }

impl From<Song> for Text<'_> {
    fn from(value: Song) -> Self {
        return Text::raw(value.title)
    }
}
impl From<Song> for String {
    fn from(value: Song) -> Self {
        return value.title
    }
}
pub struct Queue  {
    pub songs: Vec<Song>,
    // playing_song: i32,
    pub state: ListState
}

impl Queue {
    // pub fn new<T>(controller: &'b mut Sink, songs: Vec<Song>) -> Self {
    pub fn new<T>() -> Self {
        // let mut items: Vec<ListItem> = Vec::new();
        let songs = [
            Song::new("./music/Rolling-Contact-One-Night-In-Imperishable.mp3".to_owned()),
            Song::new("./music/Rolling-Contact-Words-of-Yesterday.mp3".to_owned())
        ];
        // for s in songs { // create list of songs displayed in the queue tab
        //     let mut txt = ratatui::text::Text::raw("");

        //     txt.push_span(format!("{}\n", s.title.clone()).bold());
        //     txt.push_span(" - ");
        //     txt.push_span(format!("{}", s.artist.clone()).gray());

        //     items.push( ListItem::new( txt ) );
        // }

        // let block =  Block::new()
        //     .border_type(ratatui::widgets::BorderType::Rounded)
        //     .title("[Q]ueue")
        //     .borders(Borders::all())
        //     .border_style(Style::new().red()); // define the border around the queue
        Queue {
            // controller: &mut Sink::try_new(&OutputStream::try_default().unwrap().1).unwrap(),
            // ui_list: List::new(items).block(block),
            // controller: controller,
            // playing_song: -1,
            songs: songs.to_vec(),
            state: ListState::default()
        }
    }
}