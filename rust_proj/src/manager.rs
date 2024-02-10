mod account_page;
mod start;
mod details_page;
// mod menu;
pub mod templates;

pub use start::get_start_page;
pub use details_page::get_details_home;
pub use details_page::post_details_home;
pub use account_page::get_account_page;
