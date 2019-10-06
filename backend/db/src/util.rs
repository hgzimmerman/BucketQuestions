//! Provides generic helper functions for manipulating database rows.
use diesel::{
    associations::HasTable,
    delete,
    dsl::Find,
    helper_types::Update,
    insertable::Insertable,
    pg::PgConnection,
    query_builder::{AsChangeset, DeleteStatement, InsertStatement, IntoUpdateTarget},
    query_dsl::{filter_dsl::FindDsl, LoadQuery, RunQueryDsl},
    query_source::QuerySource,
    result::QueryResult,
};
use uuid::Uuid;

/// Creates a row for an arbitrary table.
pub fn create_row<Model, NewModel, Table, Values>(
    table: Table,
    model_to_insert: NewModel,
    connection: &PgConnection,
) -> QueryResult<Model>
where
    NewModel: Insertable<Table, Values = Values>,
    InsertStatement<Table, Values>: LoadQuery<PgConnection, Model>,
{
    model_to_insert
        .insert_into(table)
        .get_result::<Model>(connection)
}

/// Updates a generic row.
#[inline(always)]
#[allow(dead_code)]
pub fn update_row<Model, Chg, Tab>(
    table: Tab,
    changeset: Chg,
    conn: &PgConnection,
) -> QueryResult<Model>
where
    Chg: AsChangeset<Target = <Tab as HasTable>::Table>,
    Tab: QuerySource + IntoUpdateTarget,
    Update<Tab, Chg>: LoadQuery<PgConnection, Model>,
{
    diesel::update(table)
        .set(changeset)
        .get_result::<Model>(conn)
}

/// Generic function for getting a whole row from a given table.
#[inline(always)]
pub fn get_row<Model, Table>(table: Table, uuid: Uuid, conn: &PgConnection) -> QueryResult<Model>
where
    Table: FindDsl<Uuid>,
    Find<Table, Uuid>: LoadQuery<PgConnection, Model>,
{
    table.find(uuid).get_result::<Model>(conn)
}

/// Generic function for deleting a row from a given table.
#[inline(always)]
pub fn delete_row<Model, Tab>(table: Tab, uuid: Uuid, conn: &PgConnection) -> QueryResult<Model>
where
    Tab: FindDsl<Uuid>,
    <Tab as FindDsl<Uuid>>::Output: IntoUpdateTarget,
    DeleteStatement<
        <<Tab as FindDsl<Uuid>>::Output as HasTable>::Table,
        <<Tab as FindDsl<Uuid>>::Output as IntoUpdateTarget>::WhereClause,
    >: LoadQuery<PgConnection, Model>,
{
    delete(table.find(uuid)).get_result::<Model>(conn)
}
