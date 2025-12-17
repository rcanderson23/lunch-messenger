use std::fmt::Write;

use chrono::Days;
use clap::Parser;

use crate::config::Cli;
use crate::nutrislice::get_menu;

mod config;
mod nutrislice;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let today = chrono::offset::Local::now().date_naive();
    let tomorrow = today.checked_add_days(Days::new(1)).unwrap();

    let menu = get_menu(
        "helenaschools",
        "jim-darcy-elementary-school",
        nutrislice::MenuType::Lunch,
        &tomorrow,
    )
    .await?;

    let mut message = String::new();
    for day in &menu.days {
        if chrono::NaiveDate::parse_from_str(&day.date, "%Y-%m-%d").unwrap() != tomorrow {
            continue;
        }

        let foods: Vec<String> = day
            .menu_items
            .iter()
            .filter_map(|mi| mi.food.clone())
            .map(|f| f.name)
            .collect();
        if foods.is_empty() {
            continue;
        }
        writeln!(message, "{}", day.date)?;
        for food in foods {
            writeln!(&mut message, "{food}")?;
        }
        writeln!(message)?;
        break;
    }

    let msg =
        pushover_rs::MessageBuilder::new(&cli.pushover_user_key, &cli.pushover_token, &message)
            .build();

    match pushover_rs::send_pushover_request(msg).await {
        Ok(resp) => {
            if let Some(errors) = resp.errors {
                eprintln!("error sending pushover message");
                for e in errors {
                    eprintln!("{}", e);
                }
            }
        }
        Err(e) => eprintln!("failed to send pushover message: {}", e),
    }

    Ok(())
}
