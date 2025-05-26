use poise::serenity_prelude::{self as serenity};
use regex::Regex;
use mime::Mime;

use super::Handler;

impl Handler {
    pub async fn star_embeds(&self, ctx: serenity::Context, new_message: serenity::Message) {
        if (new_message.attachments.len() > 0) || (new_message.embeds.len() > 0) {
            new_message.react(&ctx, serenity::ReactionType::Unicode("\u{2b50}".to_string())).await.unwrap();
        } else {
            let regex = Regex::new(r#"(http|https):\/\/([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&:\/~+#-]*[\w@?^=%&\/~+#-]?)"#).unwrap();
            for (regex_match, [_protocol, _domain, _path]) in regex.captures_iter(&new_message.content).map(|c| c.extract()) {
                match reqwest::get(regex_match).await {
                    Ok(request) => {
                        if let Some(content_type) = request.headers().get(reqwest::header::CONTENT_TYPE) {
                            let mime: Mime = content_type.to_str().unwrap().parse().unwrap();
                            match mime.type_() {
                                mime::IMAGE => (),
                                mime::VIDEO => (),
                                mime::AUDIO => (),
                                _ => continue,
                            }
                            new_message.react(&ctx, serenity::ReactionType::Unicode("\u{2b50}".to_string())).await.unwrap();
                            break;
                        }
                    },
                    Err(e) => println!("Reqwest get returned err: {}", e),
                }
            }
        }
    }
}