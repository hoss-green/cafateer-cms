mod account_page;
mod start;
mod details_page;
pub mod templates;
pub mod components;

pub use start::get_start_page;
pub use details_page::get_details_home;
pub use details_page::post_details_home;
pub use account_page::get_account_page;
pub use account_page::post_language;
pub use account_page::post_primary_language;
