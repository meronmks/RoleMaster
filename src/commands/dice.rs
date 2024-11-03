use rand::Rng;
use serenity::all::{CommandOptionType, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::Integer(diceN), ..
    }) = options.get(0)
    {
        let mut rnd = rand::thread_rng();
        let num: i64 = rnd.gen_range(1..*diceN);
        format!("{}面ダイスを振りました！　結果は「{}」でした！", *diceN, num).to_string()
    } else {
        "NaN".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("dice")
        .description("1dN")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "ダイス面の数", "1～9999")
                .min_int_value(1)
                .max_int_value(9999)
                .required(true)
        )
}