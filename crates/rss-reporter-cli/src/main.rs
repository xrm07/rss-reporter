use rss_reporter_core::{load_site_context,SubscriptionArticles};

fn main() {
    match load_site_context() {
        Ok(report) => {
            for subscription_articles in report.subscriptions{
                let SubscriptionArticles {
                    subscription,
                    articles,
                } = subscription_articles;

                println!("subscription: {}", subscription.title);
                println!("site: {}", subscription.site_url);
                println!("feed: {}", subscription.feed_url);
                println!();

                for article in articles{
                    println!("  title: {}", article.title);
                    println!("  publisher: {}", article.publisher_site);
                    println!("  tag: {}", article.topic_tag);
                    println!("  url: {}", article.url);
                    println!();
                }
            }

            for error in report.errors{
                eprintln!(
                    "failed: {} ({})  {:?}",
                    error.source_name, error.feed_url, error.error
                );
            }
        }
        
        Err(err) => {
            eprintln!("failed to load config: {:?}", err);
        }
    }
}
