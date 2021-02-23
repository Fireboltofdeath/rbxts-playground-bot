use linkify::LinkFinder;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::{
    async_trait,
    model::prelude::{Activity, OnlineStatus, Ready},
};
use url::Host::Domain;

use crate::util::send_url_embed;

pub struct Handler;

// #[allow(unused_must_use)]
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        let text = &message.content;
        let link_finder = LinkFinder::new();
        let links: Vec<_> = link_finder.links(&text).collect();
        let mapped_links = links
            .iter()
            .map(|link| (url::Url::parse(&text[link.start()..link.end()]).ok(), link))
            .filter_map(|link| match link.0 {
                Some(url) => Some((url, link.1)),
                None => None,
            });
        for data in mapped_links {
            let url = data.0;
            let link = data.1;
            if let Some(Domain(domain)) = url.host() {
                if domain == "roblox-ts.com" || domain == "www.roblox-ts.com" {
                    if url.path().starts_with("/playground") {
                        if (link.start() == 0 && link.end() == text.len()) {
                            message.delete(&ctx).await.ok();
                        }
                        send_url_embed(
                            "[Shortened Playground Link]",
                            &url.to_string(),
                            &ctx,
                            &message,
                        )
                        .await;
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, _data: Ready) {
        ctx.set_presence(
            Some(Activity::playing("roblox-ts compiler")),
            OnlineStatus::Online,
        )
        .await;
    }
}
