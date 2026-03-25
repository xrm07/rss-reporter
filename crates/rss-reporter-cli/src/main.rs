use rss_reporter_core::{convert_rssxml_to_feed, feed_to_site_ctx};
fn main() {
    let urls:Vec<&str> = vec!["https://scour.ing/@xrm07/rss.xml","https://wired.jp/rssfeeder/"];

    let feeds = convert_rssxml_to_feed(urls);

    let site_contexts = feed_to_site_ctx(feeds);
}
/*
std::process::commandによるコマンド実行の返り値，Output．そのエンティティ．stdoutはコマンド実行後によって返されるもの，stderrはstdoutのエラーを受け取るものをもつ．
これらはVec<u8>で保存される．これらはバイト列なので普通にprintしようとすると数字の羅列が帰ってくる．それを加工するのがString::from_utf8_lossy,これによってutf-8にバイト列を検証しながら変換してくれる．

*/