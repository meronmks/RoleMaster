use std::collections::HashMap;
use std::env;
use std::num::NonZeroU64;
use std::sync::Arc;
use chrono::{DateTime, Duration, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use log::{debug, error, info};
use rand::prelude::SliceRandom;
use rand::SeedableRng;
use rand::seq::IteratorRandom;
use rand_chacha::ChaCha12Rng;
use serenity::all::{ButtonStyle, ChannelId, CommandInteraction, CommandOptionType, Context, CreateActionRow, CreateButton, CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage, EmojiId, Message, MessageId, ReactionType, ResolvedOption, ResolvedValue, RoleId, User, UserId};
use serenity::all::TeamMemberRole::Admin;
use serenity::Error;
use serenity::model::Permissions;
use crate::commands;
use crate::job_manager::JobManager;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use crate::db_manager::{DbManager, ReactionEmoji, RoleAssignment, UserMissCount, Users};

pub async fn run(ctx: Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    if let Some(ResolvedOption {
        value: ResolvedValue::Integer(instanceNumb), ..
    }) = interaction.data.options().get(0){
        let date_str:String;
        let today = Local::now().naive_local();
        if let Some(ResolvedOption {
            value: ResolvedValue::String(dateStr), ..
        }) = interaction.data.options().get(1){
            date_str = dateStr.to_string();
        } else {
            let specified_time = NaiveTime::from_hms_opt(21, 45, 0).unwrap();
            let datetime = NaiveDateTime::new(today.into(), specified_time);
            let format_str = datetime.format("%Y/%m/%d %H:%M").to_string();
            date_str = format_str;
        }
        let dt = NaiveDateTime::parse_from_str(&date_str, "%Y/%m/%d %H:%M");
        if dt.is_err() {
            let data = CreateInteractionResponseMessage::new().content(format!("エラー：日付の書式が間違っています。\n入力された文字列：{}", date_str)).ephemeral(true);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = interaction.create_response(&ctx.http, builder).await {
                error!("Error sending message: {:?}", why);
            }
            return Ok(());
        }
        let date_time = dt.unwrap_or_default();
        let duration: Duration = date_time - today;

        if duration.num_minutes() <= 0 {
            let data = CreateInteractionResponseMessage::new().content(format!("エラー：入力された日付は既に過ぎているか、期限が短すぎます。\n入力された日付：{}", date_str)).ephemeral(true);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = interaction.create_response(&ctx.http, builder).await {
                error!("Error sending message: {:?}", why);
            }
            return Ok(());
        }

        let max_cap : i64;
        if let Some(ResolvedOption {
            value: ResolvedValue::Integer(num), ..
        }) = interaction.data.options().get(2){
            max_cap = *num;
        } else {
            max_cap = 20;
        }

        let channel_id :ChannelId;
        if let Some(ResolvedOption {
            value: ResolvedValue::Channel(channel), ..
        }) = interaction.data.options().get(3){
            channel_id = channel.id;
        } else {
            channel_id  = interaction.channel_id;
        }

        let local_datetime = Local.from_local_datetime(&date_time).unwrap();
        let unix_time = local_datetime.timestamp();

        let data = CreateInteractionResponseMessage::new()
            .content(format!("以下の設定で募集を開始します。\n\
            募集インスタンス数： {} \n\
            募集人数： {}\n\
            締切日時: <t:{}>", *instanceNumb, max_cap, unix_time))
            .ephemeral(true);
        let builder = CreateInteractionResponse::Message(data);
        if let Err(why) = interaction.create_response(&ctx.http, builder).await {
            error!("Error sending message: {:?}", why);
        }

        let db_pool = DbManager::get_connection();
        let reactions = sqlx::query_as::<_, ReactionEmoji>("SELECT * FROM reaction_emoji_settings ORDER BY instanceNum ASC")
            .fetch_all(&*db_pool)
            .await.unwrap();

        let m = channel_id.say(&ctx.http, format!("<t:{}:D>のエントリー受付を開始します。\n来店を希望される方は以下のリアクションより希望するインスタンス番号に該当するものをリアクションしてください！（複数インスタンスへの希望可）\n募集インスタンス数： {} \nエントリー締切日時: <t:{}>", unix_time, *instanceNumb, unix_time)).await.expect("Unable to send message");

        let mut n = 0;
        for reaction in reactions {
            if (n == *instanceNumb) {
                break;
            }
            if(reaction.isCustomEmoji == false){
                let reaction = ReactionType::try_from(reaction.unicodeEmoji).unwrap();
                m.react(&ctx.http, reaction).await.expect("Unable to react");
            } else {
                let custom_emoji = ReactionType::Custom {
                    animated: reaction.isAnimatedCustomEmoji,
                    id: EmojiId::new(reaction.customEmojiId),
                    name: Some(reaction.customEmojiName),
                };
                m.react(&ctx.http, custom_emoji).await.expect("Unable to react custom_emoji");
            }
            n += 1;
        }

        let guild_id = interaction.clone().guild_id.clone();
        let instance_num = instanceNumb.clone();

        JobManager::add_one_shot_job(format!("{:?}/{:?}/{:?}/{:?}",interaction.guild_id, channel_id, m.id, date_str).as_str(), core::time::Duration::from_secs(duration.num_seconds() as u64),  move || {
            let ctx = ctx.clone();
            let m = m.clone();

            tokio::spawn(async move {
                let db_pool = DbManager::get_connection();
                let message = refresh_message(&ctx, &m).await.unwrap();
                let mut all_entry_user: HashMap<String, Vec<User>> = HashMap::new(); // インスタンス毎の抽選用リスト
                for reaction in &message.reactions {
                    if reaction.me == false {
                        continue;
                    }
                    let emoji = &reaction.reaction_type;
                    let users = get_all_reaction_users(&ctx, &message, emoji.clone()).await.unwrap();
                    for user in users {
                        if user.bot == true || user.system == true {
                            continue;
                        }

                        // TODO:ロール等で除外する処理をここに入れる

                        let user_miss_count = sqlx::query_as::<_, UserMissCount>("SELECT * FROM user_miss_count WHERE discordId = $1")
                            .bind(user.id.get() as i64)
                            .fetch_optional(&*db_pool)
                            .await.unwrap();

                        match user_miss_count{
                            Some(row) => {
                                // 今までの挑戦数に応じて多めにリストに入れる
                                for i in 0..row.count{
                                    if i % 2 == 0 {
                                        all_entry_user.entry(emoji.to_string()).or_insert(vec![]).push(user.clone());
                                    }
                                }
                            }
                            None => {
                                // 一回だけ入れる
                                all_entry_user.entry(emoji.to_string()).or_insert(vec![]).push(user.clone());
                            }
                        }

                        let user_row = sqlx::query_as::<_, Users>("SELECT * FROM users WHERE discordId = $1")
                            .bind(user.id.get() as i64)
                            .fetch_optional(&*db_pool)
                            .await.unwrap();

                        match user_row {
                            Some(row) => {
                                sqlx::query("UPDATE users SET updateAt = $1 WHERE discordId = $2")
                                    .bind(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())
                                    .bind(user.id.get() as i64)
                                    .execute(&*db_pool)
                                    .await.unwrap();
                            }
                            None => {
                                sqlx::query("INSERT INTO users (discordId, userName, displayName) values ($1, $2, $3)")
                                    .bind(user.id.get() as i64)
                                    .bind(user.name.clone())
                                    .bind(user.display_name().clone())
                                    .execute(&*db_pool)
                                    .await.unwrap();
                            }
                        }
                    }
                }

                // 抽選準備が整ったはずなので第一インスタンスから抽選開始
                info!("抽選開始");
                let reactions = sqlx::query_as::<_, ReactionEmoji>("SELECT * FROM reaction_emoji_settings ORDER BY instanceNum ASC")
                    .fetch_all(&*db_pool)
                    .await.unwrap();
                let mut n = 0;
                let mut rng = ChaCha12Rng::from_entropy();
                for reaction in reactions {
                    if (n == instance_num) {
                        break;
                    }

                    let role_assignment = sqlx::query_as::<_, RoleAssignment>("SELECT * FROM role_assignment WHERE instanceNum = $1")
                        .bind(n+1)
                        .fetch_one(&*db_pool)
                        .await.unwrap();

                    let mut key:String;
                    if(reaction.isCustomEmoji == false){
                        key = reaction.unicodeEmoji;
                    } else{
                        key = format!("<:{}:{}>",reaction.customEmojiName, reaction.customEmojiId);
                    }

                    let popped_values = random_pop_individual(&mut all_entry_user, &key, max_cap as usize);

                    // 抽選で当たった人
                    for u in popped_values {
                        guild_id.unwrap().member(&ctx.http, u.id).await.unwrap().add_role(&ctx.http, RoleId::new(role_assignment.roleId)).await.unwrap();
                    }

                    n += 1;
                }

                info!("抽選終了");

                // 最後まで残ってた人（惜しくも抽選に外れた人）の処理

                let role_assignment = sqlx::query_as::<_, RoleAssignment>("SELECT * FROM role_assignment WHERE instanceNum = $1")
                    .bind(0)
                    .fetch_one(&*db_pool)
                    .await.unwrap();

                for (_, value) in all_entry_user {
                    for u in value {
                        let user_miss_count = sqlx::query_as::<_, UserMissCount>("SELECT * FROM user_miss_count WHERE discordId = $1")
                            .bind(u.id.get() as i64)
                            .fetch_optional(&*db_pool)
                            .await.unwrap();

                        match user_miss_count{
                            Some(row) => {
                                sqlx::query("UPDATE user_miss_count SET count = $1, updateAt = $2 WHERE discordId = $3")
                                    .bind(row.count + 1)
                                    .bind(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())
                                    .bind(u.id.get() as i64)
                                    .execute(&*db_pool)
                                    .await.unwrap();
                            }
                            None => {
                                sqlx::query("INSERT INTO user_miss_count (discordId, count) values ($1, $2)")
                                    .bind(u.id.get() as i64)
                                    .bind(1)
                                    .execute(&*db_pool)
                                    .await.unwrap();
                            }
                        }

                        guild_id.unwrap().member(&ctx.http, u.id).await.unwrap().add_role(&ctx.http, RoleId::new(role_assignment.roleId)).await.unwrap();
                    }
                }

                //guild_id.unwrap().member(&ctx.http, user.id).await.unwrap().remove_role(&ctx.http, RoleId::new(1295618567798132778)).await.unwrap();
                // let test = channel_id.get_thread_members(&ctx.http).await.unwrap();
                // for t in test {
                //     t.member.unwrap().remove_role(&ctx.http, "").await.unwrap();
                // }
            });

        }).await.unwrap();
    }

    Ok(())
}

fn random_pop_individual<K: Eq + std::hash::Hash + Clone>(
    map: &mut HashMap<K, Vec<User>>,
    key: &K,
    count: usize,
) -> Vec<User> {
    let mut result = Vec::new();

    if let Some(mut values) = map.remove(key) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            if let Some(item) = values.iter().choose(&mut rng).cloned() {
                result.push(item.clone());
                values.retain(|x| x != &item);

                for (_, other_values) in map.iter_mut() {
                    other_values.retain(|x| x != &item);
                }
            }
        }

        if !values.is_empty() {
            map.insert(key.clone(), values);
        }
    }

    result
}

async fn refresh_message(ctx: &Context, message: &Message) -> Option<Message> {
    match message.channel_id.message(&ctx.http, message.id).await {
        Ok(updated_message) => Some(updated_message),
        Err(err) => {
            error!("Failed to retrieve message: {:?}", err);
            None
        },
    }
}

async fn get_all_reaction_users(
    ctx: &Context,
    message: &Message,
    emoji: ReactionType,
) -> Result<Vec<User>, Error> {
    let mut users_list = Vec::new();
    let mut after = None;

    loop {
        let users = message
            .channel_id
            .reaction_users(
                &ctx.http,
                message.id,
                emoji.clone(),
                Some(100),
                after,
            )
            .await?;

        for user in &users {
            users_list.push(user.clone());
        }

        if users.len() < 100 {
            break;
        }

        after = users.last().map(|user| user.id);
    }

    Ok(users_list)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("entry").description("エントリー受付用の投稿をbotから行います")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "インスタンス数", "1～10")
                .min_int_value(1)
                .max_int_value(10)
                .required(true)
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "集計締切日時", "YYYY/MM/DD HH:MM　で指定、すべて省略すると当日21:45　例）2024/03/26 22:50")
                .required(false)
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "1インスタンスあたりの当選者数", "デフォルトは20人")
                .min_int_value(1)
                .max_int_value(80)
                .required(false)
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Channel, "募集投稿をするチャンネル", "デフォルトはこのコマンドを実行したチャンネル")
                .required(false)
        )
}