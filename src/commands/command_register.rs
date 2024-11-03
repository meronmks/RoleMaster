use log::error;
use serenity::all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage, Permissions};
use crate::commands;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    if let Err(why) = interaction.guild_id.unwrap().set_commands(&ctx.http, vec![
        commands::ping::register(),
        commands::dice::register(),
        commands::entry::register(),
        commands::list_entry::register(),
        commands::cancel_entry::register(),
    ]).await {
        error!("Error commands registered fail: {:?}", why);
    }
    let data = CreateInteractionResponseMessage::new().content("コマンドの登録が完了しました。").ephemeral(true);
    let builder = CreateInteractionResponse::Message(data);
    if let Err(why) = interaction.create_response(&ctx.http, builder).await {
        error!("Error sending message: {:?}", why);
    }
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("command_register").description("[bot管理用] コマンドを登録します")
        .default_member_permissions(Permissions::ADMINISTRATOR)
}