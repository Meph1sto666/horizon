use std::collections::HashMap;
use tui_tree_widget::TreeItem;
use crate::playlist::{dir_to_songs, Song};

pub struct Library {
	// pub state: TreeState<&'a str>,
	// pub state: &'a mut TreeState<&'a String>,
	pub tree_entries: Vec<TreeItem<'static, String>>,
	pub songs: Vec<Song>
}

impl Clone for Library {
	fn clone(&self) -> Self {
        Library {
			songs: self.songs.clone(),
			tree_entries: self.tree_entries.clone(),
		}
    }
}

impl Library {
	// pub fn new(state: &'a mut TreeState<&'a String>) -> Self {
	pub fn new() -> Self {
		Self {
            songs: Vec::new(),
			tree_entries: Vec::new(),
        }
	}
	
	pub fn update_tree_entries(&mut self) {
		let mut root: HashMap<String, HashMap<String, Vec<Song>>> = HashMap::new();
		let song_list = &dir_to_songs(&"./music/".to_owned());
		// song_list.into_iter().filter(|w| matches(*w.artist, w1)).collect::<Vec<Word>>()
		for song in song_list {
			let mut root_clone = root.clone();
			if !root_clone.contains_key(&song.artist.clone()) {root_clone.insert(song.artist.clone(), HashMap::new());}
			let artist_content: &mut HashMap<String, Vec<Song>> = root_clone.get_mut(&(song.artist.clone())).unwrap();
			if !artist_content.contains_key(&song.album.clone()) {artist_content.insert(song.album.clone(), Vec::new());}
			let mut album_content: Vec<Song> = artist_content.get(&(song.album.clone())).unwrap().clone();

			album_content.push(song.clone());
			artist_content.insert(song.album.clone(), album_content);
			root.insert(song.artist.clone(), artist_content.clone());
		}
		
		let mut items: Vec<TreeItem<'_, String>> = Vec::new(); // = TreeItem::new_leaf("l", "leaf");

		for artist in root.keys() {
			let mut artist_items: Vec<TreeItem<'_, String>> = Vec::new(); // = TreeItem::new_leaf("l", "leaf");
			for album in root.get(artist).expect("no such key in artist hashmap").keys() {
				let mut album_items: Vec<TreeItem<'_, String>> = Vec::new(); // = TreeItem::new_leaf("l", "leaf");
				for song in root.get(artist).unwrap().get(album).unwrap() {
					album_items.push(TreeItem::new_leaf(song.path.to_string(), song.title.to_string()));
				}
				artist_items.push(TreeItem::new(album.to_string(), album.to_string(), album_items).unwrap());
			}
			items.push(TreeItem::new(artist.to_string(), artist.to_string(), artist_items).unwrap());
		}
		self.tree_entries = items;
	}
}
