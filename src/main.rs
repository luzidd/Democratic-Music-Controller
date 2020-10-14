extern crate mpd;

use mpd::Client;
use std::net::TcpStream;
use std::os::unix::net::UnixStream;

fn main() {
//    let sock = UnixStream::connect("/path/to/my/socket");
    let sock = TcpStream::connect("127.0.0.1:6600");
    let sock = match sock {
        Ok(sock) => sock,
        Err(error) => {
                        println!("\nTHERE WAS A PROOOOBLEM opening the socket...\n");
                        return },
    };
    let mut conn = Client::new(sock).unwrap();
    println!("Music Directory: {:?}", conn.music_directory());
    println!("Rescan: {:?}", conn.rescan());
    println!("Mounts: {:?}", conn.mounts());
    println!("\n\nSuccess!");
}
