use feed_rs::model::{Feed};
use std::{process::Command};
use feed_rs::parser;

#[derive(Debug)]
pub struct SiteCtx {
    pub site_name:String,
    pub article_title:String,
    pub topic_tag:String,
    pub article_url:String,
}

impl SiteCtx {
    pub fn nomalize_scour_url(&mut self){
        if self.article_url.starts_with("https://scour.ing/redirect/"){
            self.article_url = self.article_url.strip_prefix("https://scour.ing/redirect/").unwrap().to_string();
            self.article_url = self.article_url.strip_suffix("?utm_source=rss").unwrap().to_string();
        }
    }
}

pub fn convert_rssxml_to_feed(urls:Vec<&str>) -> Vec<Feed>{
    let mut feeds:Vec<Feed> = Vec::new();
    for url in urls{
        let xml_output = Command::new("curl").arg(url).output().unwrap_or_else(|e| panic!("Failed to use curl to receive XML from {url}: {e}"));
        if !xml_output.status.success(){
            eprint!(
                "curl failed for {url}: status={}, stderr={}",
                xml_output.status,
                String::from_utf8_lossy(&xml_output.stderr)
            );
        }else{
            let feed = parser::parse(&xml_output.stdout[..]).expect("Parse error from xml to feed");
            feeds.push(feed);
        }
    }
    feeds
}

pub fn feed_to_site_ctx(feeds:Vec<Feed>) -> Vec<SiteCtx>{
    let mut vec_site_ctx:Vec<SiteCtx> = Vec::new();
    for feed in feeds{
        for current in feed.entries{
            let mut modified = SiteCtx{
                site_name:feed.title.clone().map(|t| t.content).unwrap_or_default(),
                article_title:current.title.map(|t| t.content).unwrap_or_default(),
                topic_tag:current.categories.first()
                                            .map(|c| c.term.clone())
                                            .unwrap_or_else(|| "not existed".to_string()),
                article_url:current.links
                                            .first()
                                            .map(|l| l.href.clone())
                                            .unwrap_or_else(|| "".to_string()),
            };
            modified.nomalize_scour_url();
            vec_site_ctx.push(modified);
        }
    }
    vec_site_ctx
}