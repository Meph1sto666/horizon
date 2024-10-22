use tui_tree_widget::{TreeState, Tree, TreeItem};
// mod playlist;
use crate::playlist::{dir_to_songs, Song};

pub struct Library<'a> {
	pub state: TreeState<&'a str>,
	// pub state: TreeState<String>,
	pub songs: Vec<Song>
}

impl<'a> Library<'a> {
	pub fn new<T>() -> Self {
        Self {
            state: TreeState::default(),
			songs: Vec::new(),
        }
    }

	// pub fn create_tree(&mut self) -> TreeItem<'a, &'a str>{
	// 	let mut b: TreeItem<'a, & str> = TreeItem::new("r", "Root", vec![]).unwrap();
	// 	for song in &self.songs {
    //         b.add_child(TreeItem::new_leaf(&song.path, song.title.to_string())).unwrap();
    //     }
	// 	return b;
	// }
}