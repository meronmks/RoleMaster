use serenity::all::Permissions;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "Pong!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("[bot管理用] botが生きてたら「Pong!」と返します")
        .default_member_permissions(Permissions::ADMINISTRATOR)
}