use log::error;
use serenity::{all::{CommandInteraction, Context, EditInteractionResponse, Permissions, RoleId}, builder::CreateCommand};

use crate::db_manager::{DbManager, RoleAssignment};

fn get_report_interval(total_members: usize) -> usize {
    match total_members {
        0..=100 => 10,
        101..=1000 => 100,
        1001..=2000 => 200,
        _ => 500,
    }
}

pub async fn run(ctx: Context, interaction: &CommandInteraction) -> String {
    if let Err(e) = interaction.defer_ephemeral(&ctx.http).await {
        error!("応答の遅延に失敗しました: {:?}", e);
        return "コマンドの実行に失敗しました。".to_string();
    }

    let guild_id = interaction.clone().guild_id.unwrap();
    let mut all_members = Vec::new();
    let mut last_member_id = None;
    let db_pool = DbManager::get_connection();

    loop {
        let members = match guild_id.members(&ctx.http, Some(1000), last_member_id).await {
            Ok(members) => members,
            Err(e) => {
                error!(
                    "メンバー一覧の取得に失敗しました。Guild ID: {}, Error: {:?}",
                    guild_id, e
                );
                return "メンバー一覧の取得に失敗しました。".to_string();
            }
        };

        if members.is_empty() {
            break;
        }

        last_member_id = members.last().map(|m| m.user.id);
        all_members.extend(members);

        if last_member_id.is_none() {
            break;
        }
    }

    let roles_to_remove = match sqlx::query_as::<_, RoleAssignment>(
        "SELECT * FROM role_assignment"
    )
    .fetch_all(&*db_pool)
    .await {
        Ok(roles) => roles,
        Err(e) => {
            error!("ロール情報の取得に失敗しました。Error: {:?}", e);
            return "ロール情報の取得に失敗しました。".to_string();
        }
    };

    let total_members = all_members.len();
    let mut processed = 0;
    let report_interval = get_report_interval(total_members);

    for member in all_members {
        for role in &roles_to_remove {
            let role_id = RoleId::new(role.role_id);
            if member.roles.contains(&role_id) {
                if let Err(e) = member.remove_role(&ctx.http, role_id).await {
                    error!("ロールの削除に失敗しました。 {}: {:?}", member.user.name, e);
                    continue;
                }
            }
        }

        processed += 1;
        if processed % report_interval == 0 {
            if let Err(e) = interaction
                .edit_response(&ctx.http, EditInteractionResponse::new().content(format!("処理中... ({}/{}人完了)", processed, total_members)))
                .await
            {
                error!("進捗報告の送信に失敗しました: {:?}", e);
            }
        }
    }

    if let Err(e) = interaction
        .edit_response(&ctx.http, EditInteractionResponse::new().content("処理が完了しました。"))
        .await
    {
        error!("応答の編集に失敗しました: {:?}", e);
        return "応答の送信に失敗しました。".to_string();
    }

    "処理が完了しました。".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("reset_roles")
        .description("このbotで振り分けたロールを全員から外します。")
        .default_member_permissions(Permissions::ADMINISTRATOR)
}