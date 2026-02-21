use feed_rs::model::{Feed, Text};

#[derive(Debug)]
pub struct SiteCtx {
    pub site_name:String,
    pub article_title:String,
    pub topic_tag:String,
    pub article_url:String,
}

pub fn feed_to_SiteCtx(feed:Feed) -> Vec<SiteCtx>{
    let mut vec_SiteCtx:Vec<SiteCtx> = Vec::new();
    for current in feed.entries{
        let modified = SiteCtx{
            site_name:feed.title.clone().map(|t| t.content).unwrap_or_default(),
            article_title:current.title.map(|t| t.content).unwrap_or_default(),
            topic_tag:current.categories.get(0)
                                        .map(|c| c.term.clone())
                                        .unwrap_or_else(|| "not existed".to_string()),
            article_url:current.links
                                        .get(0)
                                        .map(|l| l.href.clone())
                                        .unwrap_or_else(|| "".to_string()),
        };
        vec_SiteCtx.push(modified);
    }
    vec_SiteCtx
}