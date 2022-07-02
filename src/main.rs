
fn main() {

    let size=3;

    //initialize SongInfo with empty strings
    let mut song_info = std::iter::repeat_with(|| SongInfo::new(&"".to_string(),&"".to_string()))
    .take(size)
    .collect::<Vec<_>>();

    
    for i in 0..size{
        let mut song = String::new();
        let mut title = String::new();
        println!("Enter song :");
        let b1 = std::io::stdin().read_line(&mut song).unwrap();
        let b2 = std::io::stdin().read_line(&mut title).unwrap();
        get_song_info(&mut song_info[i],song,title);
    }
    print_song_info(song_info);

}

#[derive(Clone, Debug)]
pub struct SongInfo{
    pub artist: String,
    pub title: String,
}
impl SongInfo{
    pub fn new(artist:&String, title:&String)->SongInfo{
        SongInfo{
            artist:artist.clone(),
            title:title.clone()
        }
    }
}
pub fn get_song_info(songInfo:&mut SongInfo,artist:String,title:String){
    songInfo.artist= artist.trim().to_string();
    songInfo.title= title.trim().to_string();
}
pub fn print_song_info(songInfos:Vec<SongInfo>){
    for info in songInfos{
        println!("{0:width$}{1}",info.artist,info.title,width=40);
    }
}