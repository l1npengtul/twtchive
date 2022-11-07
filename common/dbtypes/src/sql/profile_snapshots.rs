use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user: u64,
    pub display: String,
    pub handle: String,
    pub bio: String,
    pub profession: String,
    pub joined: DateTime,
    pub birthday: DateTime,
    pub profile: String,
    pub snapshot_id: u64,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        todo!()
    }
}

impl ActiveModelBehavior for ActiveModel {}
