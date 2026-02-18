use feed_rs::model::{Feed, Text};

#[derive(Debug)]
pub struct site_ctx {
    pub site_name:String,
    pub article_title:Text,
    pub topic_tag:String,
    pub article_url:String,
}

pub fn feed_to_site_ctx(feed:Feed) -> Vec<site_ctx>{
    let mut vec_site_ctx:Vec<site_ctx> = Vec::new();
    for current in feed.entries{
        let modified = site_ctx{
            site_name:feed.title.clone().unwrap().content,
            article_title:current.title.unwrap(),
            topic_tag:current.categories.get(0)
                                        .map(|c| c.term.clone())
                                        .unwrap_or_else(|| "not existed".to_string()),
            article_url:current.links
                                        .get(0)
                                        .map(|l| l.href.clone())
                                        .unwrap_or_else(|| "".to_string()),
        };
        vec_site_ctx.push(modified);
    }
    vec_site_ctx
}