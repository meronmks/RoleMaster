use serenity::all::{CommandInteraction, CommandOptionType, CreateCommandOption, Permissions, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use crate::job_manager::JobManager;

pub async fn run(options: &[ResolvedOption<'_>]) -> String {
    if let Some(ResolvedOption {
                    value: ResolvedValue::String(job_id), ..
                }) = options.get(0)
    {
        if let Err(why) = JobManager::cancel_job(*job_id).await {
            return "指定されたJobIDはありません。".to_string();
        }
        "指定されたJobはキャンセルされました。".to_string()
    } else {
        "引数が不正です。".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("cancel_entry")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .description("（管理用）botが正式なentryとして認識しているやつの取消")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "job_id", "キャンセルしたい対象のjob")
                .required(true)
        )
}