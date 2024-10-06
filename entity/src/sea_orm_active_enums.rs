//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "month")]
pub enum Month {
    #[sea_orm(string_value = "april")]
    April,
    #[sea_orm(string_value = "august")]
    August,
    #[sea_orm(string_value = "december")]
    December,
    #[sea_orm(string_value = "february")]
    February,
    #[sea_orm(string_value = "january")]
    January,
    #[sea_orm(string_value = "july")]
    July,
    #[sea_orm(string_value = "june")]
    June,
    #[sea_orm(string_value = "march")]
    March,
    #[sea_orm(string_value = "may")]
    May,
    #[sea_orm(string_value = "november")]
    November,
    #[sea_orm(string_value = "october")]
    October,
    #[sea_orm(string_value = "september")]
    September,
}
