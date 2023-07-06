pub mod barter;
pub mod collections_floor;
pub mod conversation;
pub mod generic_nft;
pub mod nft;
pub mod werewolf;

#[macro_export]
macro_rules! barter_yard_db {
    ($type:ty, $name: literal) => {
        impl ModelCollection for $type {
            fn get_db_name() -> String {
                "BarterYard".to_string()
            }
            fn get_col_name() -> String {
                $name.to_string()
            }
        }
    };
}
