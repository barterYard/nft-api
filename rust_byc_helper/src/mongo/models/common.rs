use mongodb::Client;

pub trait ModelCollection
where
    Self: Sized,
{
    fn get_col_name() -> String;
    fn get_db_name() -> String;
    fn get_collection(client: &Client) -> mongodb::Collection<Self> {
        client
            .database(Self::get_db_name().as_str())
            .collection::<Self>(Self::get_col_name().as_str())
    }
}
