//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "bills")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub owner: i32,
    pub driver: Option<i32>,
    pub image: i32,
    pub status: i8,
    pub price: Option<i32>,
    pub reason: Option<String>,
    pub handled_by: Option<i32>,
    pub faction: i8,
    pub target_faction: Option<i8>,
    pub date: DateTimeUtc,
    pub modified: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
