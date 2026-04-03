use crate::integrations::{ProPresenter, Vmix};
use dioxus::prelude::*;
use std::collections::BTreeMap;
use std::str::FromStr;
#[cfg(feature = "server")]
use tokio::time::{Duration, sleep};

macro_rules! simple_file_sql {
    ($name : ident, $param : ty) => {
        #[server]
        pub async fn $name(param: $param) -> Result<(), anyhow::Error> {
            let _ = DB.with(|f| {
                f.execute(
                    include_str!(concat!("sql/", stringify!($name), ".sql")),
                    [&param],
                )
            })?;
            Ok(())
        }
    };
    ($name : ident, $param : ty, $param2 : ty) => {
        #[server]
        pub async fn $name(param: $param, param2: $param2) -> Result<(), anyhow::Error> {
            let _ = DB.with(|f| {
                f.execute(
                    include_str!(concat!("sql/", stringify!($name), ".sql")),
                    (&param, &param2),
                )
            })?;
            Ok(())
        }
    };
}

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("./db/database.db").expect("Failed to open database");
        let _ = conn.execute_batch(include_str!("sql/migrate.sql"));
        conn
    };
    pub static CLIENT: reqwest::Client = reqwest::Client::new();
}

#[server]
pub async fn get_announcements() -> Result<Vec<(i32, String, String)>, ServerFnError> {
    let rows = DB.with(|f| {
        f.prepare(include_str!("sql/get_announcements.sql"))
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .unwrap()
            .map(|row| row.unwrap())
            .collect::<Vec<(i32, String, String)>>()
    });

    Ok(rows)
}

simple_file_sql!(add_announcement, String, String);
simple_file_sql!(edit_announcement_name, String, i32);
simple_file_sql!(edit_announcement_text, String, i32);
simple_file_sql!(remove_announcement, i32);

#[server]
pub async fn show_announcement(announcement: String) -> Result<(), ServerFnError> {
    let pp = get_propresenter().await?;

    if let Err(e) = pp.add_message(announcement).await {
        error!("Add Error: {}", e);
    };

    sleep(Duration::from_millis(500)).await;

    if let Err(e) = pp.trigger_message().await {
        error!("Trigger Error: {}", e);
    };

    Ok(())
}

#[server]
pub async fn clear_announcement() -> Result<(), ServerFnError> {
    let pp = get_propresenter().await?;

    if let Err(e) = pp.clear_message().await {
        error!(e);
    };

    if let Err(e) = pp.remove_message().await {
        error!(e);
    };

    Ok(())
}

#[server]
pub async fn get_propresenter() -> Result<ProPresenter, ServerFnError> {
    let (url, message_name, theme_name, theme_index, theme_uuid): (
        String,
        String,
        String,
        i32,
        String,
    ) = DB.with(|f| {
        f.prepare(include_str!("sql/get_propresenter.sql"))
            .unwrap()
            .query_row([], |row| {
                Ok((
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            })
            .unwrap()
    });
    let pro_presenter = ProPresenter::new(
        url,
        message_name,
        theme_name,
        theme_index.to_string(),
        theme_uuid,
    );
    Ok(pro_presenter)
}

#[server]
pub async fn set_propresenter(pp: ProPresenter) -> Result<(), anyhow::Error> {
    let params = (
        pp.get_pro_presenter_url(),
        pp.get_message_name(),
        pp.get_theme_name(),
        i32::from_str(&pp.get_theme_index())?,
        pp.get_theme_uuid(),
    );
    let _ = DB.with(|f| f.execute(include_str!("sql/set_propresenter.sql"), params))?;
    Ok(())
}

#[server]
pub async fn get_bauchbinden() -> Result<BTreeMap<i32, (String, Vec<(i32, String)>)>, ServerFnError>
{
    let mut map = BTreeMap::new();
    let rows = DB.with(|f| {
        f.prepare(include_str!("sql/get_bauchbinden.sql"))
            .unwrap()
            .query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })
            .unwrap()
            .map(|row| row.unwrap())
            .collect::<Vec<(i32, String, Option<i32>, Option<String>)>>()
    });

    for (section_id, section_name, person_id, person_name) in rows {
        if !map.contains_key(&section_id) {
            map.insert(section_id, (section_name, Vec::new()));
        }
        if let Some(name) = person_name {
            map.get_mut(&section_id)
                .unwrap()
                .1
                .push((person_id.unwrap(), name));
        }
    }

    Ok(map)
}

#[server]
pub async fn set_bauchbinde_text(field: String, value: String) -> Result<(), ServerFnError> {
    let vmix = get_vmix().await?;

    vmix.set_text(value, field)
        .await
        .expect("Unable to set name_field");

    Ok(())
}

#[server]
pub async fn show_bauchbinde(name: String, section: String) -> Result<(), ServerFnError> {
    info!("show bauchbinde \"{}: {}\"", name, section);

    let vmix = get_vmix().await?;

    vmix.set_text(name, vmix.get_name_field())
        .await
        .expect("Unable to set name_field");
    vmix.set_text(section, vmix.get_title_field())
        .await
        .expect("Unable to set title_field");
    vmix.overlay_input()
        .await
        .expect("Unable to show bauchbinde");

    Ok(())
}

simple_file_sql!(add_section, String);
simple_file_sql!(edit_section, String, i32);

#[server]
pub async fn remove_section(id: i32) -> Result<(), anyhow::Error> {
    let _ = DB.with(|f| f.execute(include_str!("sql/remove_names.sql"), [&id]))?;
    let _ = DB.with(|f| f.execute(include_str!("sql/remove_section.sql"), [&id]))?;
    Ok(())
}

simple_file_sql!(add_name, String, i32);
simple_file_sql!(edit_name, String, i32);
simple_file_sql!(remove_name, i32);

#[server]
pub async fn get_vmix() -> Result<Vmix, ServerFnError> {
    let (vmix_url, overlay_index, object_uuid, name_field, title_field): (
        String,
        u8,
        String,
        String,
        String,
    ) = DB.with(|f| {
        f.prepare(include_str!("sql/get_vmix.sql"))
            .unwrap()
            .query_row([], |row| {
                Ok((
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            })
            .unwrap()
    });
    let vmix = Vmix::new(
        vmix_url,
        overlay_index.to_string(),
        object_uuid,
        name_field,
        title_field,
    )
    .unwrap();
    Ok(vmix)
}

#[server]
pub async fn get_vmix_titles() -> Result<BTreeMap<String, (String, Vec<String>)>, ServerFnError> {
    let vmix = get_vmix().await?;
    let res = vmix.get_vmix_titles().await;
    match res {
        Ok(v) => Ok(v),
        Err(e) => Err(ServerFnError::ServerError {
            message: e,
            code: 500,
            details: None,
        }),
    }
}

#[server]
pub async fn set_vmix(vmix: Vmix) -> Result<(), anyhow::Error> {
    let params = (
        vmix.get_vmix_url(),
        u8::from_str(&vmix.get_overlay_index())?,
        vmix.get_object_uuid(),
        vmix.get_name_field(),
        vmix.get_title_field(),
    );
    let _ = DB.with(|f| f.execute(include_str!("sql/set_vmix.sql"), params))?;
    Ok(())
}
