use crate::util::*;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::{client::Context, framework::standard::Args};

#[command("playground")]
pub async fn to_playground(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut code = args.rest();
    if code.starts_with("```ts") && code.ends_with("```") {
        code = &code[5..code.len() - 3].trim();
    }
    let url = format!("https://roblox-ts.com/playground/#code/{}", compress(code));

    send_url_embed("[Playground Link]", &url, ctx, msg).await;
    msg.delete(&ctx).await.ok();
    Ok(())
}
