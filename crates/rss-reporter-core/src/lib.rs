use config::File;
use feed_rs::model::{Entry, Feed, Link};
use reqwest::{StatusCode, Url};
use serde::Deserialize;

// config.tomlに存在するSourceを保持
#[derive(Debug, Deserialize)]
struct RssSource {
    name: String,
    feed_url: String,
    enabled: bool,
}

// config.tomlで有効になっているSourceをVecで保持
#[derive(Debug, Deserialize)]
struct EnabledRssSources {
    source: Vec<RssSource>,
}

//ユーザーが本当に対処できるエラー表示だけをまずは完成させる．
//(URLが不正，xmlが送られてこない，サーバーの応答が遅い, エラーコードまでにする)
//TODO:このツールの対象ユーザーを確定，明確化した後，エラーを出す範囲を確定させる．
#[derive(Debug)]
pub enum RequestError {
    Config(String),
    InvalidUrl(String),
    TimeOut,
    InvalidXml,
    HttpStatus(StatusCode),
    Other(String),
}

//購読しているサイトの情報
#[derive(Debug, Clone)]
pub struct SubscriptionCtx {
    pub title: String,
    pub site_url: String,
    pub feed_url: String,
}

// 記事の情報
#[derive(Debug)]
pub struct ArticleCtx {
    pub publisher_site: String,
    pub title: String,
    pub url: String,
    pub topic_tag: String,
}

// 購読しているRSS
#[derive(Debug)]
pub struct SubscriptionArticles {
    pub subscription: SubscriptionCtx,
    pub articles: Vec<ArticleCtx>,
}

// 購読している有効なRSSとそのエラーを持つ
#[derive(Debug)]
pub struct LoadArticleContextReport {
    pub subscriptions: Vec<SubscriptionArticles>,
    pub errors: Vec<SourceError>,
}

//RSSのエラーの詳細情報
#[derive(Debug)]
pub struct SourceError {
    pub source_name: String,
    pub feed_url: String,
    pub error: RequestError,
}

pub fn load_site_context() -> Result<LoadArticleContextReport, RequestError> {
    let sources = load_enabled_sources().map_err(|err| RequestError::Config(err.to_string()))?;

    // リクエストを行うクライアントを作成，リクエストそれぞれでこのクライアントを再利用する．
    let client = reqwest::blocking::Client::new();

    let mut subscriptions: Vec<SubscriptionArticles> = Vec::new();
    let mut errors = Vec::new();

    for source in sources {
        match load_one_site_context(&source, &client) {
            Ok(subscription_articles) => subscriptions.push(subscription_articles),
            Err(err) => errors.push(SourceError {
                source_name: source.name.clone(),
                feed_url: source.feed_url.clone(),
                error: err,
            }),
        }
    }
    Ok(LoadArticleContextReport {
        subscriptions,
        errors,
    })
}

//登録されているRSS一つからサイト群を取り出し，RSSを提供しているサイト名とともにSubscriptionArticlesに保存する
fn load_one_site_context(
    source: &RssSource,
    client: &reqwest::blocking::Client,
) -> Result<SubscriptionArticles, RequestError> {
    let url = Url::parse(&source.feed_url)
        .map_err(|_| RequestError::InvalidUrl(source.feed_url.clone()))?;

    let xml = fetch_rssxml(url.clone(), client)
        .map_err(|err| classify_caused_error_with_reqwest(err, url))?;

    let feed = parse_rssxml_to_feed(xml).map_err(|_| RequestError::InvalidXml)?;

    // let mut subscription_articles = parse_feed_to_site_ctx(feed);
    // subscription_articles.normalize_scour_url();

    Ok(parse_feed_to_subscription_articles(feed, source))
}

// Project 直下のconfig.tomlを読んで，enable = Trueのサイト情報だけをVec<RssSouce>として返す．
fn load_enabled_sources() -> Result<Vec<RssSource>, config::ConfigError> {
    let configuration = config::Config::builder()
        .add_source(File::with_name("config").required(true))
        .build()?;

    let app: EnabledRssSources = configuration.try_deserialize()?;

    Ok(app.source.into_iter().filter(|s| s.enabled).collect())
}

// webサイトにXMLを取りに行き，Result型でreqwestのエラーと中身をで受け取る．
fn fetch_rssxml(url: Url, client: &reqwest::blocking::Client) -> Result<String, reqwest::Error> {
    client.get(url).send()?.error_for_status()?.text()
}

//reqwestが引き起こしたエラーを確認し，振り分けて返す．
fn classify_caused_error_with_reqwest(err: reqwest::Error, url: Url) -> RequestError {
    if err.is_timeout() {
        RequestError::TimeOut
    } else if err.is_status() {
        RequestError::HttpStatus(err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
    } else {
        RequestError::Other(format!("{}: {}", url, err))
    }
}

//xmlをfeedにパースする．（feed_rs::parser）
fn parse_rssxml_to_feed(xml: String) -> Result<Feed, feed_rs::parser::ParseFeedError> {
    feed_rs::parser::parse(xml.as_bytes())
}

//feedをSubscriptionArticlesにパースする．
fn parse_feed_to_subscription_articles(feed: Feed, source: &RssSource) -> SubscriptionArticles {
    let subscription = parse_feed_to_subscription_ctx(&feed,source);

    let articles = feed
        .entries
        .into_iter()
        .map(|entry| parse_entry_to_article_ctx(entry, &subscription))
        .collect();

    SubscriptionArticles {
        subscription,
        articles,
    }
}

fn parse_feed_to_subscription_ctx(feed: &Feed, source: &RssSource) -> SubscriptionCtx {
    SubscriptionCtx {
        title: feed
            .title
            .as_ref()
            .map(|title| title.content.clone())
            .unwrap_or_else(|| source.name.clone()),
        site_url: primary_link_href(&feed.links).unwrap_or_else(|| source.feed_url.clone()),
        feed_url: source.feed_url.clone(),
    }
}

fn parse_entry_to_article_ctx(entry: Entry, subscription: &SubscriptionCtx) -> ArticleCtx {
    let raw_url = primary_link_href(&entry.links).unwrap_or_else(|| entry.id.clone());
    let url = nomalize_artilcle_url(&raw_url);

    let title = entry
        .title
        .as_ref()
        .map(|title| title.content.clone())
        .unwrap_or_else(|| entry.id.clone());

    let topic_tag = entry
        .categories
        .first()
        .map(|category| category.term.clone())
        .unwrap_or_default();

    let publisher_site = publisher_site_from_url(&url, &subscription.title);

    ArticleCtx {
        publisher_site,
        title,
        url,
        topic_tag,
    }
}

fn primary_link_href(links: &[Link]) -> Option<String> {
    links
        .iter()
        .find(|link| !link.href.trim().is_empty())
        .map(|link| link.href.clone())
}

fn publisher_site_from_url(url: &str, fallback: &str) -> String {
    Url::parse(url)
        .ok()
        .and_then(|url| url.host_str().map(|host| host.to_owned()))
        .unwrap_or_else(|| fallback.to_owned())
}
fn nomalize_artilcle_url(url: &str) -> String {
    extract_scour_redirect_taget(url).unwrap_or_else(|| url.to_owned())
}

fn extract_scour_redirect_taget(url:&str) -> Option<String>{
    let parsed = Url::parse(url).ok()?;

    if parsed.host_str()? != "scour.ing" {
        return None;
    }

    let encoded_target = parsed.path().strip_prefix("/redirect/")?;
    let decoded_target = percent_encoding::percent_decode_str(encoded_target)
        .decode_utf8()
        .ok()?
        .into_owned();

    Url::parse(&decoded_target).ok()?;

    Some(decoded_target)
}