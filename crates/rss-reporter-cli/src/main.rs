use feed_rs::parser;
use std::process::Command;
use std::io::Cursor;

fn main() {
    let out = Command::new("curl").arg("https://scour.ing/@xrm07/rss.xml").output().expect("Failed to execute process");
    if out.status.success() == true {
        let feed = parser::parse(Cursor::new(out.stdout.as_slice()));
        println!("{:?}",feed.unwrap().title.unwrap());
    }else{
        println!("{:?}",out.stderr);
        return;
    }
}
