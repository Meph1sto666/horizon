use rascii_art::{
    render_to,
    RenderOptions,
};

pub fn to_ascii(file: String, size: u32) -> String {
    let mut buffer: String = String::new();
    render_to(
        file,
        &mut buffer,
        &RenderOptions::new()
            .width(size)
            .colored(true)
            // .charset(&[".", "+","?","Q","#","@", "~"]),
            .charset(&["#"]),

			// .charset(&["$","@","B","%","8","&","W","M","#","*","o","a","h","k","b","d","p","q","w","m","Z","O","0","Q","L","C","J","U","Y","X","z","c","v","u","n","x","r","j","f","t","|","1","?","-","_","+","~","<","i","!","l","I",";",":",",","^","`","'","."])
			// .charset(&["$","@","B","8","W","M","*","o","a","h","k","b","d","p","q","w","m","Z","O","0","Q","L","C","J","U","Y","X","z","c","v","u","n","x","r","j","f","t","|","1","?","-","_","+","~","<","i","!","l","I",";",":",",","^","`","'","."])
    ).unwrap_or_default();
	return buffer;
}