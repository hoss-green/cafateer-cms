mod account_page;
pub mod components;
mod details_page;
pub mod macro_templates;
mod start;
pub mod templates;

pub use account_page::get_account_page;
pub use account_page::post_language;
pub use account_page::post_primary_language;
pub use details_page::get_details_data;
pub use details_page::get_details_home;
pub use details_page::post_details_home;
pub use start::get_start_page;
