use std::fmt::Display;

use chrono::NaiveDate;
use serde::Deserialize;

//https://helenaschools.nutrislice.com/menu/jim-darcy-elementary-school/breakfast
pub async fn get_menu(
    district: &str,
    school: &str,
    menu: MenuType,
    date: &NaiveDate,
) -> anyhow::Result<Menu> {
    let date_path = date_to_path(date);
    let uri = http::Uri::builder()
        .scheme("https")
        .authority(format!("{district}.api.nutrislice.com"))
        .path_and_query(format!(
            "/menu/api/weeks/school/{school}/menu-type/{menu}/{date_path}"
        ))
        .build()?;
    let menu: Menu = reqwest::get(uri.to_string()).await?.json().await?;

    Ok(menu)
}

fn date_to_path(date: &NaiveDate) -> String {
    date.format("%Y/%m/%d").to_string()
}

#[derive(Debug, Clone)]
pub enum MenuType {
    Lunch,
}

impl Display for MenuType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            MenuType::Lunch => f.write_str("lunch"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Menu {
    pub days: Vec<Day>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Day {
    pub date: String,
    pub menu_items: Vec<MenuItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MenuItem {
    pub food: Option<Food>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Food {
    pub name: String,
}
