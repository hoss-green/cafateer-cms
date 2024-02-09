use crate::data_models::reference_items::Language;

use self::context::AppState;

pub mod context;
pub mod details;
pub mod fetcher;
pub mod menu_items;
pub mod references;

struct Counter {
    count: Option<i64>,
}

pub async fn setup_db(app_state: &AppState) {
    let setup_langs = vec![
        Language {
            id: 0,
            code: "en".to_string(),
            name: "English".to_string(),
        },
        Language {
            id: 1,
            code: "es".to_string(),
            name: "Spanish".to_string(),
        },
        Language {
            id: 2,
            code: "fr".to_string(),
            name: "French".to_string(),
        },
        Language {
            id: 3,
            code: "de".to_string(),
            name: "German".to_string(),
        },
    ];

    let result = sqlx::query_as!(Counter, "Select count(id) as count from ref_languages")
        .fetch_one(&app_state.database_pool)
        .await;
    let lang_count: i64 = match result {
        Ok(r) => {
            let actual_count = r.count.unwrap_or(0);
            println!("number of languages: {}", actual_count);
            actual_count
        }
        Err(_) => panic!("Could not access database to count number of languages"),
    };

    if lang_count >= setup_langs.len() as i64 {
        return;
    };

    let res = sqlx::query!("truncate ref_languages")
        .execute(&app_state.database_pool)
        .await;
    match res {
        Ok(_) => (),
        Err(_) => panic!("Could not remove languages from database to rebuild"),
    };

    // "insert into details(id, lang, blurb) VALUES ($1, $2, $3) ON CONFLICT (id, lang) DO UPDATE SET blurb=$3 WHERE details.id=$1 and details.lang=$2",
    for lang in setup_langs {
        let res = sqlx::query!(
            "insert into ref_languages (id, code, name) VALUES ($1, $2, $3)",
            lang.id,
            lang.code,
            lang.name
        )
        .execute(&app_state.database_pool)
        .await;

        match res {
            Ok(_) => (),
            Err(_) => panic!("Could not add languages to database error, failing"),
        }
    }
}
