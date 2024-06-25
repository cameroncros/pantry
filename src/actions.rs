use diesel::prelude::*;
use diesel::result::Error::DatabaseError;

use crate::models;
use crate::models::Item;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_item(
    conn: &mut SqliteConnection,
    uid: i32,
) -> Result<Option<models::Item>, DbError> {
    use crate::schema::items::dsl::*;
    loop {
        let result = items
            .filter(id.eq(uid))
            .first::<models::Item>(conn)
            .optional();
        return match result {
            Ok(item) => {
                Ok(item)
            }
            Err(error) => {
                match &error {
                    DatabaseError(_, desc) => {
                        if desc.details() == Some("database is locked") {
                            continue
                        }
                        Err(Box::from(error))
                    }
                    _ => {
                        Err(Box::from(error))
                    }
                }
            }
        }
    }
}

pub fn new_item(
    conn: &mut SqliteConnection,
) -> Result<Item, DbError> {
    use crate::schema::items::dsl::*;

    loop {
        let result = diesel::insert_into(items).values(description.eq("")).get_result(conn);
        return match result {
            Ok(item) => {
                Ok(item)
            }
            Err(error) => {
                match &error {
                    DatabaseError(_, desc) => {
                        if desc.details() == Some("database is locked") {
                            continue
                        }
                        Err(Box::from(error))
                    }
                    _ => {
                        Err(Box::from(error))
                    }
                }
            }
        }
    }
}

pub fn update_item(
    conn: &mut SqliteConnection,
    u_id: &i32,
    nm: &Item
) -> Result<Item, DbError> {
    use crate::schema::items::dsl::*;

    let mut new_item = nm.clone();
    new_item.id = *u_id;

    loop {
        let result = diesel::insert_into(items).values(&new_item).on_conflict(id).do_update()
            .set(description.eq(&nm.description))
            .get_result(conn);
        return match result {
            Ok(item) => {
                Ok(item)
            }
            Err(error) => {
                match &error {
                    DatabaseError(_, desc) => {
                        if desc.details() == Some("database is locked") {
                            continue
                        }
                        Err(Box::from(error))
                    }
                    _ => {
                        Err(Box::from(error))
                    }
                }
            }
        }
    }
}

pub fn delete_item(
    conn: &mut SqliteConnection,
    u_id: &i32
) -> Result<Item, DbError> {
    use crate::schema::items::dsl::*;

    loop {
        let result = diesel::delete(items.filter(id.eq(u_id)))
            .get_result(conn);
        return match result {
            Ok(item) => {
                Ok(item)
            }
            Err(error) => {
                match &error {
                    DatabaseError(_, desc) => {
                        if desc.details() == Some("database is locked") {
                            continue
                        }
                        Err(Box::from(error))
                    }
                    _ => {
                        Err(Box::from(error))
                    }
                }
            }
        }
    }
}
