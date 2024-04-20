use crate::manager::templates::macros::LanguageSelectionDropdownVm;
use crate::manager::templates::toggle_buttons::{DisableButton, EnableButton};
use crate::manager::templates::view_models::PrimaryLanguageVm;
use crate::{
    data_context::{context::AppState, references::get_languages},
    models::data::{reference_items::Language, ClaimsModel, ProfileLanguagesModel},
    session::claims::Claims,
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    response::Html,
    Extension,
};
use http::StatusCode;

pub async fn set_primary_language(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Html<String>) {
    let database_pool = &app_state.database_pool;
    let languages = get_languages(database_pool).await;
    let account_languages: Vec<ProfileLanguagesModel> =
        crate::data_context::manager::profile_languages::get_all(database_pool, &claims.sub)
            .await
            .into_iter()
            .filter(|acl| acl.published)
            .collect::<Vec<ProfileLanguagesModel>>();

    let _success = crate::data_context::manager::profile::set(&app_state.database_pool, &claims.sub, id).await;
    let primary_lang = Language::get_from_int(&languages, id);

    let primary_dropdown = LanguageSelectionDropdownVm {
        user_languages: account_languages
            .iter()
            .map(|lm| PrimaryLanguageVm {
                id: lm.id,
                title: Language::get_from_int(&languages, lm.language).name,
                code: lm.language,
                published: lm.published,
                primary: lm.language == primary_lang.id,
            })
            .collect::<Vec<PrimaryLanguageVm>>(),
    };
    let page: String = primary_dropdown.render().unwrap().to_string();
    (StatusCode::OK, Html(page))
}

pub async fn activate_language(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let database_pool = &app_state.database_pool;
    let language_model: ProfileLanguagesModel = ProfileLanguagesModel {
        id: uuid::Uuid::new_v4(),
        owner_id: claims.sub,
        language: id,
        published: false,
    };
    let _ =
        crate::data_context::manager::profile_languages::add(database_pool, &language_model)
            .await;
    let button: EnableButton = EnableButton {
        post_url: format!("/manager/config/language/enable/{}", language_model.id), //.to_string(),
        button_text: "Enable".to_string(),
    };

    Html(button.render().unwrap()).into_response()
}

pub async fn enable_language(
    Extension(claims): Extension<Claims<ClaimsModel>>,

    State(app_state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let database_pool = &app_state.database_pool;
    let _ =
        crate::data_context::manager::profile_languages::enable(database_pool, &claims.sub, &id)
            .await;

    let button: DisableButton = DisableButton {
        post_url: format!("/manager/config/language/disable/{}", id), //.to_string(),
        button_text: "Disable".to_string(),
    };

    Html(button.render().unwrap()).into_response()
}

pub async fn disable_language(
    Extension(claims): Extension<Claims<ClaimsModel>>,
    State(app_state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let database_pool = &app_state.database_pool;
    let _ =
        crate::data_context::manager::profile_languages::disable(database_pool, &claims.sub, &id)
            .await;
    let button: EnableButton = EnableButton {
        post_url: format!("/manager/config/language/enable/{}", id), //.to_string(),
        button_text: "Enable".to_string(),
    };

    Html(button.render().unwrap()).into_response()
}
