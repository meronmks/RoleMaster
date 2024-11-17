use serenity::all::{CommandInteraction, Permissions};
use serenity::builder::CreateCommand;
use crate::job_manager::JobManager;

pub async fn run(interaction: &CommandInteraction) -> String {
    let mut response = String::new();
    let jobs = JobManager::get_scheduled_jobs(Some(format!("{:?}", interaction.guild_id).as_str())).await;
    response.push_str("以下のjobが予約されています。\n");
    for job in jobs {
        response.push_str(&format!("`{}`\n", job));
    }

    response
}

pub fn register() -> CreateCommand {
    CreateCommand::new("list_entry")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .description("（管理用）botが正式なentryとして認識しているやつらの一覧")
}