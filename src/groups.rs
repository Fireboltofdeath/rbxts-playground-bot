use serenity::framework::standard::macros::group;

use super::commands::*;
#[group]
#[commands(to_playground)]
pub struct General;
