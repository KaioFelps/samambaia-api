
use entities::user::Model as UserModel;
use entities::user::ActiveModel as UserActiveModel;
use entities::sea_orm_active_enums::Role as UserRole;
use sea_orm::DbErr;
use uuid::Uuid;

pub trait UserRepositoryTrait {
    async fn create(&self, nickname: String, password: String, role: UserRole) -> Result<UserModel, DbErr>;

    async fn find_by_nickname(&self, nickname: &String) -> Result<Option<UserModel>, DbErr> ;

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<UserModel>, DbErr>;

    async fn save(&self, user: &UserActiveModel) -> Result<(), DbErr>;
}