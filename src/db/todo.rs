use crate::{models::{db::{DBPool, DbModel}, error::Error::{self, *}, todo::{Todo, TodoRequest, TodoUpdateRequest}}};

type Result<T> = std::result::Result<T, Error>;

const TABLE: &str = "todo";
const SELECT_FIELDS: &str = "id, name, created_at, checked";

#[derive(Clone)]
pub struct TodoModel {
    db_pool: DBPool,
}

impl DbModel for TodoModel {
    fn get_pool(&self) -> DBPool {
        self.db_pool.clone()
    }
}

impl TodoModel {
    pub fn new(db_pool: DBPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(&self, body: TodoRequest) -> Result<Todo> {
        let conn = self.get_db_con().await?;
        let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE);
        let row = conn
            .query_one(query.as_str(), &[&body.name])
            .await
            .map_err(DBQueryError)?;
        Ok(Todo::from(row))
    }

    pub async fn fetch(&self, search: Option<String>) -> Result<Vec<Todo>> {
        let con = self.get_db_con().await?;
        let where_clause = match search {
            Some(_) => "WHERE name like $1",
            None => "",
        };
        let query = format!(
            "SELECT {} FROM {} {} ORDER BY created_at DESC",
            SELECT_FIELDS, TABLE, where_clause
        );
        let q = match search {
            Some(v) => con.query(query.as_str(), &[&v]).await,
            None => con.query(query.as_str(), &[]).await,
        };
        let rows = q.map_err(DBQueryError)?;

        Ok(rows.into_iter().map(Todo::from).collect())
    }

    pub async fn update(&self, id: i32, body: TodoUpdateRequest) -> Result<Todo> {
        let con = self.get_db_con().await?;
        let query = format!(
            "UPDATE {} SET name = $1, checked = $2 WHERE id = $3 RETURNING *",
            TABLE
        );
        let row = con
            .query_one(query.as_str(), &[&body.name, &body.checked, &id])
            .await
            .map_err(DBQueryError)?;
        Ok(Todo::from(row))
    }

    pub async fn delete(&self, id: i32) -> Result<u64> {
        let con = self.get_db_con().await?;
        let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
        con.execute(query.as_str(), &[&id])
            .await
            .map_err(DBQueryError)
    }
}
