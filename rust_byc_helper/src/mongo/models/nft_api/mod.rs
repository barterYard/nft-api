use std::error::Error;

use mongodb::{options::IndexOptions, Client, IndexModel};

use super::{common::ModelCollection, mongo_doc, Owner, Transfer};

pub mod contract;
pub mod nft;
pub mod owner;
pub mod transfer;

pub async fn create_schema(m_client: &Client) -> Result<(), Box<dyn Error>> {
    let nft_col = nft::Nft::get_collection(m_client);
    nft_col.drop(None).await?;
    nft_col
        .create_index(
            IndexModel::builder()
                .keys(mongo_doc! {
                  "contract": 1
                })
                .options(IndexOptions::builder().sparse(true).build())
                .build(),
            None,
        )
        .await?;
    nft_col
        .create_index(
            IndexModel::builder()
                .keys(mongo_doc! {
                  "id": 1,
                  "contract": 1
                })
                .build(),
            None,
        )
        .await?;

    let tra_col = Transfer::get_collection(m_client);
    tra_col.drop(None).await?;
    tra_col
        .create_index(
            IndexModel::builder()
                .keys(mongo_doc! {
                    "date": 1,
                    "nft": 1,
                    "from": 1,
                    "to": 1
                })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
            None,
        )
        .await?;
    tra_col
        .create_index(
            IndexModel::builder()
                .keys(mongo_doc! {
                    "nft": 1,
                })
                .build(),
            None,
        )
        .await?;
    tra_col
        .create_index(
            IndexModel::builder()
                .keys(mongo_doc! {
                    "from": 1,
                })
                .build(),
            None,
        )
        .await?;
    tra_col
        .create_index(
            IndexModel::builder()
                .keys(mongo_doc! {
                    "to": 1,
                })
                .build(),
            None,
        )
        .await?;
    let owners_col = Owner::get_collection(m_client);
    owners_col.drop(None).await?;
    owners_col
        .create_index(
            IndexModel::builder()
                .keys(mongo_doc! {
                    "address": 1,
                })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
            None,
        )
        .await?;
    Ok(())
}
