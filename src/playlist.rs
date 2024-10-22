use std::{fs::{self, File}, io::BufReader};
// use color_eyre::owo_colors::OwoColorize;
use ratatui::{style::Stylize, text::Text, widgets::ListState};
use rodio::Decoder;
use symphonia::core::{formats::FormatOptions, io::{MediaSourceStream, MediaSourceStreamOptions}, meta::{Limit, MetadataOptions}};
use symphonia::default::get_probe;


pub struct Song {
    pub path: String,
	pub title: String,
	pub artist: String,
    pub album: String,
    pub track_num: String,
    pub album_tracks_total: String,
    pub year: String
}

impl Song {
    pub fn new(path: String) -> Self {
        let file: File = File::open(&path).expect(&format!("Failed to open file {}", path.clone()));
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
        // return Text::raw(value.title)
        let mut txt = ratatui::text::Text::raw("");
        txt.push_span(format!("{}\n", value.title.clone()).bold());
        txt.push_span(" - ");
        txt.push_span(format!("{}", value.artist.clone()));
        return txt
    }
}
impl From<Song> for String {
    fn from(value: Song) -> Self {
        return value.title
    }
}
pub struct Queue  {
    pub songs: Vec<Song>,
    pub state: ListState
}

impl Queue {
    pub fn new<T>() -> Self {
        // let mut items: Vec<ListItem> = Vec::new();
        // for s in songs { // create list of songs displayed in the queue tab
        //     let mut txt = ratatui::text::Text::raw("");

        //     txt.push_span(format!("{}\n", s.title.clone()).bold());
        //     txt.push_span(" - ");
        //     txt.push_span(format!("{}", s.artist.clone()).gray());

        //     items.push( ListItem::new( txt ) );
        // }

        Queue {
            songs: Vec::new(),
            state: ListState::default()
        }
    }

    pub fn push_from_path(&mut self, path: &String) {
        self.songs.push(Song::new(path.clone()));
    }

    // pub fn go_to() {
        
    // }
}
pub fn dir_to_songs(dir_path: &String) -> Vec<Song> {
    let paths: fs::ReadDir = fs::read_dir(dir_path).unwrap();
    let mut songs: Vec<Song> = Vec::new();
    for p in paths {
        songs.push(Song::new(p.unwrap().path().display().to_string()));
    }
    return songs;
}