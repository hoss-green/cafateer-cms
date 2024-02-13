mod account_page;
pub mod components;
mod details_page;
pub mod macro_templates;
pub mod menu_page;
mod start;
pub mod templates;

pub use account_page::get_account_page;
pub use account_page::post_language;
pub use account_page::post_primary_language;
pub use details_page::get_details_data;
pub use details_page::get_details_home;
pub use details_page::post_details_home;
pub use menu_page::get_menu_page;
pub use menu_page::get_menu_item;
pub use menu_page::set_menu_item;
pub use start::get_start_page;
