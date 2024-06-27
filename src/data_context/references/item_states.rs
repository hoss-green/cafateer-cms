pub enum ItemStates {
    Deactivated,
    Draft,
    Published,
}

impl ItemStates {
    pub fn get_enum(num: i64) -> ItemStates {
        match num {
            0 => ItemStates::Deactivated,
            1 => ItemStates::Draft,
            2 => ItemStates::Published,
            _ => panic!("Tried to recover out of range ItemState")
        }
    }
}
