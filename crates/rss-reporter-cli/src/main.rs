use std::process::Command;
use feed_rs::parser;
fn main() {
    let out = Command::new("curl").arg("https://scour.ing/@xrm07/rss.xml").output().expect("Failed to execute process");
    if out.status.success(){
        let xml = String::from_utf8_lossy(&out.stdout).to_string();
        let feed = parser::parse(xml.as_bytes()).unwrap();
        let vec_site_ctx = rss_reporter_core::feed_to_site_ctx(feed);
        println!("{:?}",vec_site_ctx);
        for ctx in vec_site_ctx{
            println!("--------------------");
            println!("{:?}: tag:{}",ctx.article_title.content,ctx.topic_tag);
            println!("link: {}",ctx.article_url);
            println!("--------------------");
        }
    }else{
        println!("{:?}",out.stderr);
        return;
    }
}
/*
std::process::commandによるコマンド実行の返り値，Output．そのエンティティ．stdoutはコマンド実行後によって返されるもの，stderrはstdoutのエラーを受け取るものをもつ．
これらはVec<u8>で保存される．これらはバイト列なので普通にprintしようとすると数字の羅列が帰ってくる．それを加工するのがString::from_utf8_lossy,これによってutf-8にバイト列を検証しながら変換してくれる．

*/