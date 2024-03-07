use crate::{
    data_context::{context::AppState, references::get_languages},
    manager::components::PrimaryLanguageList,
    models::data::{reference_items::Language, ProfileLanguagesModel},
};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
};
use http::StatusCode;

pub async fn post_language(
    State(app_state): State<AppState>,
    body: String,
) -> (StatusCode, Html<String>) {
    println!("{:#?}", body);
    let mut profile = crate::data_context::manager::profile::get(&app_state.database_pool).await;
    let lang_setting = match body.contains("&") {
        true => match body.split("&").last() {
            Some(body) => match body.split("=").last() {
                Some(id) => (id.to_string().parse::<i32>().unwrap(), true),
                None => return (StatusCode::OK, Html("error".to_string())),
            },
            None => return (StatusCode::OK, Html("error".to_string())),
        },
        false => match body.split("=").last() {
            Some(id) => (id.to_string().parse::<i32>().unwrap(), false),
            None => todo!(),
        },
    };

    match lang_setting.1 {
        true => {
            crate::data_context::manager::profile_languages::add(
                &app_state,
                &ProfileLanguagesModel {
                    id: uuid::Uuid::new_v4(),
                    owner_id: profile.id,
                    language: lang_setting.0,
                },
            )
            .await
        }
        false => {
            crate::data_context::manager::profile_languages::delete(&app_state, profile.id, lang_setting.0)
                .await
        }
    };

    let account_languages =
        crate::data_context::manager::profile_languages::get_all(&app_state, profile.id)
            .await
            .iter()
            .map(|al| al.language)
            .collect::<Vec<i32>>();
    let account_languages = match account_languages.len() {
        0 => {
            let am = ProfileLanguagesModel {
                    id: uuid::Uuid::new_v4(),
                    owner_id: profile.id,
                    language: 0,
                };
            crate::data_context::manager::profile_languages::add(
                &app_state,
                &am
            )
            .await;
            vec![0]
        }
        _ => account_languages,
    };
    if !account_languages.iter().any(|&al| al == profile.primary_language) {
        profile.primary_language = *account_languages.iter().last().unwrap_or(&0);
        crate::data_context::manager::profile::set(&app_state.database_pool, &profile).await;
    }
    let languages = get_languages(&app_state).await;
    let primary_dropdown = PrimaryLanguageList {
        primary_language_id: profile.primary_language,
        user_selected_languages: Language::vec_from_int_vec(&languages, &account_languages),
    };
    let page: String = primary_dropdown.render().unwrap().to_string();
    (StatusCode::OK, Html(page))
}

pub async fn post_primary_language(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Html<String>) {
    let languages = get_languages(&app_state).await;
    let mut profile = crate::data_context::manager::profile::get(&app_state.database_pool).await;
    let account_languages =
        crate::data_context::manager::profile_languages::get_all(&app_state, profile.id)
            .await
            .iter()
            .map(|al| al.language)
            .collect::<Vec<i32>>();
    profile.primary_language = id;
    crate::data_context::manager::profile::set(&app_state.database_pool, &profile).await;
    let primary_dropdown = PrimaryLanguageList {
        primary_language_id: profile.primary_language,
        user_selected_languages: Language::vec_from_int_vec(&languages, &account_languages),
    };
    let page: String = primary_dropdown.render().unwrap().to_string();
    (StatusCode::OK, Html(page))
}
