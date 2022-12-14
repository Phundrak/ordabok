use super::super::schema::{userfollows, users};
use diesel::prelude::*;

use crate::db::Database;

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[juniper::graphql_object(Context = Database)]
impl User {
    #[graphql(description = "Appwrite ID of the user")]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[graphql(description = "The user's apparent name")]
    pub fn username(&self) -> String {
        self.username.clone()
    }

    #[graphql(description = "Who the user follows")]
    pub fn following(&self, context: &Database) -> Vec<User> {
        use super::super::schema::{userfollows, users};
        let conn = &mut context.conn().unwrap();
        userfollows::dsl::userfollows
            .filter(userfollows::dsl::follower.eq(self.id.clone()))
            .load::<UserFollow>(conn)
            .unwrap()
            .iter()
            .map(|f| {
                users::dsl::users
                    .find(f.following.clone())
                    .first::<User>(conn)
                    .unwrap()
            })
            .collect::<Vec<User>>()
    }
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = userfollows)]
pub struct UserFollow {
    pub id: i32,
    pub follower: String,
    pub following: String,
}
