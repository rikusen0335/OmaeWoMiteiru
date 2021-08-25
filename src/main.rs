
use std::env;

use reqwest::StatusCode;
use reqwest::header::AUTHORIZATION;
use serde_json::{Number, json};
use serenity::framework::standard::{CommandResult};
use serenity::framework::standard::macros::command;
use songbird::{SerenityInit, ffmpeg};
use serenity::client::Context;
use serenity::{async_trait, framework::{StandardFramework, standard::macros::group}, model::{channel::Message, gateway::Ready}, prelude::*};
use xshell::cmd;
use serde::Deserialize;
extern crate base64;
use base64::decode;
use std::fs::OpenOptions;
use std::io::Write;

struct Handler;

static COMMAND_PREFIX: &str = "r/";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if let Some(channel) = message.channel_id.to_channel_cached(&ctx.cache).await.unwrap().guild() {
            if channel.name.contains("聞き専")
                && !message.content.starts_with(COMMAND_PREFIX)
                && !message.author.bot
            {
                let filename = "/tmp/audio.mp3";

                // If request voice text is exists, we don't have to request it to the API, should use cached voice instead
                generate_voice(&message.content).await;

                let source = ffmpeg(filename)
                    .await
                    .expect("再生するための音声ファイルが見つかりませんでした");

                let manager = songbird::get(&ctx).await
                    .expect("Songbird Voice client placed in at initialisation.").clone();

                if let Some(handler_lock) = manager.get(message.guild_id.unwrap()) {
                    let mut handler = handler_lock.lock().await;

                    handler.play_source(source);

                    println!("「{}」を再生中", message.content);
                } else {
                    println!("「{}」を再生できませんでした", message.content);
                }
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[derive(Deserialize, Debug)]
struct Timepoint {
    markName: String,
    timeSeconds: Number,
}

#[derive(Deserialize, Debug)]
struct AudioConfig {
    audioEncoding: String,
}

#[derive(Deserialize, Debug)]
struct GoogleTTSResponse {
    audioContent: String,
    timepoints: Vec<Timepoint>,
    audioConfig: AudioConfig,
}

async fn generate_voice(text: &str) {
    let url = "https://texttospeech.googleapis.com/v1beta1/text:synthesize";

    let input = json!({
        "text": text,
    });

    let voice = json!({
        "languageCode": "ja-JP",
        "name": "ja-JP-Wavenet-B",
        "ssmlGender": "FEMALE",
    });

    let audio_config = json!({
        "audioEncoding": "MP3",
        "pitch": 0,
        "speakingRate": 1,
    });

    let request_body = json!({
        "input": input,
        "audioConfig": audio_config,
        "voice": voice,
    });

    let token = cmd!("gcloud auth application-default print-access-token").read().expect("An error occured while obtaining token");
    let authorization = format!("Bearer {}", token);

    let client = reqwest::Client::new();
    if let Ok(result) = client
        .post(url)
        .header(AUTHORIZATION, authorization)
        .body(request_body.to_string())
        .send()
        .await
    {
        match result.status() {
            StatusCode::OK => {
                println!("リクエスト成功");

                let result_body = result.json::<GoogleTTSResponse>().await;
                match result_body {
                    Ok(body) => generate_voice_file(body.audioContent).await,
                    Err(why) => println!("{}", why)
                }
            },
            StatusCode::UNAUTHORIZED => println!("認証情報に誤りがあります"),
            StatusCode::BAD_REQUEST => println!("リクエストが不正です"),
            StatusCode::FORBIDDEN => println!("必要な権限がありません"),
            _ => println!("Anything wrong idk goes")
        }
    }
}

async fn generate_voice_file(audio_bytes: String) {
    let filename = "/tmp/audio.mp3"; // This needs to be dynamic named using sha1 to generate every voice for caching
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename);
    match file {
        Ok(mut f) => {
            println!("ファイルに音声を書き込み中...");
            let bytes = decode(audio_bytes).unwrap();
            f.write_all(&bytes).unwrap();
            f.flush().unwrap();
            println!("ファイルに音声を書き込みました");
        },
        Err(why) => {
            println!("ファイルを作成できませんでした");
            println!("{}", why)
        }
    }
}

#[group]
#[commands(kite, bye)]
struct General;

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let _credential = env::var("GOOGLE_APPLICATION_CREDENTIALS").expect("Expected a secret credential including Google TTS");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(COMMAND_PREFIX))
        .group(&GENERAL_GROUP);

    let mut client =
        Client::builder(&token)
            .event_handler(Handler)
            .framework(framework)
            .register_songbird()
            .await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    tokio::spawn(async move {
        let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
    });

    tokio::signal::ctrl_c().await.unwrap();
    println!("Received Ctrl-C, shutting down.");
}

#[command]
#[only_in(guilds)]
async fn kite(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.reply(ctx, "先にVC入れアホ").await.unwrap();

            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn bye(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.leave(guild_id);

    Ok(())
}
