use lz_str::compress_uri;
use serenity::{client::Context, model::channel::Message};

pub async fn send_url_embed(text: &str, url: &str, ctx: &Context, message: &Message) {
    message
        .channel_id
        .send_message(ctx, |m| {
            m.embed(|embed| {
                embed.title(text);
                embed.url(url.to_string());
                embed.author(|author| {
                    if let Some(icon) = message.author.static_avatar_url() {
                        author.icon_url(icon);
                    }
                    author.name(message.author.tag());
                    author
                });
                embed
            });
            m
        })
        .await
        .ok();
}

pub fn unsigned_to_char(num: &u32) -> char {
    num.to_owned() as u8 as char
}

pub fn compress(code: &str) -> String {
    compress_uri(code)
        .iter()
        .map(unsigned_to_char)
        .collect::<String>()
}
