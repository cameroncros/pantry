use diesel::prelude::*;

use crate::models;
use crate::models::Item;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_item(
    conn: &mut SqliteConnection,
    uid: i32,
) -> Result<Option<models::Item>, DbError> {
    use crate::schema::items::dsl::*;

    let item = items
        .filter(id.eq(uid))
        .first::<models::Item>(conn)
        .optional()?;

    Ok(item)
}

pub fn new_item(
    conn: &mut SqliteConnection,
) -> Result<Item, DbError> {
    use crate::schema::items::dsl::*;

    let result = diesel::insert_into(items).values(description.eq("")).get_result(conn);

    Ok(result?)
}

pub fn update_item(
    conn: &mut SqliteConnection,
    u_id: &i32,
    nm: &Item
) -> Result<Item, DbError> {
    use crate::schema::items::dsl::*;

    let mut new_item = nm.clone();
    new_item.id = *u_id;

    let u = diesel::update(items.filter(id.eq(u_id)))
        .set(description.eq(&nm.description))
        .get_result(conn);

    Ok(u?)
}

pub fn delete_item(
    conn: &mut SqliteConnection,
    u_id: &i32
) -> Result<Item, DbError> {
    use crate::schema::items::dsl::*;

    let u = diesel::delete(items.filter(id.eq(u_id)))
        .get_result(conn);

    Ok(u?)
}
