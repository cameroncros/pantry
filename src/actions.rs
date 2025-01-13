use diesel::prelude::*;
use diesel::result::Error::DatabaseError;

use crate::models;
use crate::models::Item;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_item(conn: &mut SqliteConnection, uid: i32) -> Result<models::Item, DbError> {
    use crate::schema::items::dsl::*;
    loop {
        let result = items
            .filter(id.eq(uid))
            .first::<Item>(conn)
            .optional();
        return match result {
            Ok(item) => {
                match item {
                    None => { Ok(Item {id: uid, description: "".to_string(), date: None })}
                    Some(i) => {Ok(i)}
                }
            },
            Err(error) => match &error {
                DatabaseError(_, desc) => {
                    if desc.message() == "database is locked" {
                        continue;
                    }
                    Err(Box::from(error))
                }
                _ => Err(Box::from(error)),
            },
        };
    }
}

pub fn get_all_items(conn: &mut SqliteConnection) -> Result<Vec<Item>, DbError> {
    use crate::schema::items::dsl::*;
    loop {
        let result = items.load::<Item>(conn);
        return match result {
            Ok(all_items) => {
               Ok(all_items)
            },
            Err(error) => match &error {
                DatabaseError(_, desc) => {
                    if desc.message() == "database is locked" {
                        continue;
                    }
                    Err(Box::from(error))
                }
                _ => Err(Box::from(error)),
            },
        };
    }
}

pub fn update_item(conn: &mut SqliteConnection, u_id: &i32, nm: &Item) -> Result<Item, DbError> {
    use crate::schema::items::dsl::*;

    let mut new_item = nm.clone();
    new_item.id = *u_id;

    loop {
        let result = diesel::insert_into(items)
            .values(&new_item)
            .on_conflict(id)
            .do_update()
            .set(&new_item)
            .get_result(conn);
        return match result {
            Ok(item) => Ok(item),
            Err(error) => match &error {
                DatabaseError(_, desc) => {
                    if desc.message() == "database is locked" {
                        continue;
                    }
                    Err(Box::from(error))
                }
                _ => Err(Box::from(error)),
            },
        };
    }
}

pub fn delete_item(conn: &mut SqliteConnection, u_id: &i32) -> Result<Item, DbError> {
    use crate::schema::items::dsl::*;

    loop {
        let result = diesel::delete(items.filter(id.eq(u_id))).get_result(conn);
        return match result {
            Ok(item) => Ok(item),
            Err(error) => match &error {
                DatabaseError(_, desc) => {
                    if desc.message() == "database is locked" {
                        continue;
                    }
                    Err(Box::from(error))
                }
                _ => Err(Box::from(error)),
            },
        };
    }
}
