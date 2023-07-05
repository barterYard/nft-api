// """nfts model"""
// import mongoengine as me

// class Thumbnail(me.EmbeddedDocument):
//     cid = me.StringField()
//     path = me.StringField()
//     url = me.StringField()

// class NftDisplay(me.EmbeddedDocument):
//     description = me.StringField()
//     name = me.StringField()
//     thumbnail = me.EmbeddedDocumentField(Thumbnail)
//     edition = me.ListField(me.DictField)

// class GenericNft(me.Document):
//     _id = me.ObjectIdField(required=True, primary_key=True)
//     tokenID = me.IntField(unique_with='collection')
//     collection = me.StringField()
//     display = me.EmbeddedDocumentField(NftDisplay)

//     traits = me.ListField(me.DictField())
//     metadata = me.DictField()

use crate::{barter_yard_db, mongo::models::common::ModelCollection};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thumbnail {
    pub cid: Option<String>,
    pub path: Option<String>,
    pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NftDisplay {
    pub description: String,
    pub name: String,
    pub thumbnail: Thumbnail,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NftEdition {
    pub name: Option<String>,
    pub number: i64,
    pub max: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NftEditions {
    #[serde(rename = "infoList")]
    pub info_list: Option<Vec<NftEdition>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericNft {
    pub _id: bson::oid::ObjectId,
    pub collection: String,
    #[serde(rename = "tokenID")]
    pub token_id: u64,
    pub display: NftDisplay,
    pub traits: Option<Vec<bson::Document>>,
    pub metadata: Option<Vec<bson::Document>>,
    pub editions: Option<NftEditions>,
    pub edition: Option<NftEdition>,
}
barter_yard_db!(GenericNft, "generic_nft");
