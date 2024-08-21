//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq,Serialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub age: i32,
    pub gender: String,
    pub location: Option<String>,
    pub openness: Option<String>,
    pub interests: Option<String>,
    pub exp_qual: Option<String>,
    pub relation_type: Option<String>,
    pub social_habits: Option<String>,
    pub past_relations: Option<String>,
    pub values: Option<String>,
    pub style: Option<String>,
    pub traits: Option<String>,
    pub commitment: Option<String>,
    pub resolution: Option<String>,
    pub image_url: String,
    pub score: i32,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
