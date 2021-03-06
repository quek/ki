use super::user::*;
use arysn::prelude::*;
use async_recursion::async_recursion;
impl User {
    pub fn select() -> UserBuilder {
        UserBuilder {
            from: "users".to_string(),
            ..UserBuilder::default()
        }
    }
    pub async fn delete(&self, client: &tokio_postgres::Client) -> anyhow::Result<()> {
        client
            .execute("DELETE FROM users WHERE id = $1", &[&self.id])
            .await?;
        Ok(())
    }
    pub async fn update(&self, client: &tokio_postgres::Client) -> anyhow::Result<()> {
        client . execute ( "UPDATE users SET email = $1, name = $2, status = $3, created_at = $4, updated_at = $5 WHERE id = $6" , & [ & self . email , & self . name , & self . status , & self . created_at , & self . updated_at , & self . id ] ) . await ? ;
        Ok(())
    }
}
impl UserNew {
    pub async fn insert(&self, client: &tokio_postgres::Client) -> anyhow::Result<User> {
        let mut target_columns: Vec<&str> = vec![];
        target_columns.push(stringify!(email));
        target_columns.push(stringify!(name));
        target_columns.push(stringify!(status));
        if self.created_at.is_some() {
            target_columns.push(stringify!(created_at));
        }
        if self.updated_at.is_some() {
            target_columns.push(stringify!(updated_at));
        }
        let target_columns = target_columns.join(", ");
        let mut bind_count: i32 = 0;
        bind_count += 1;
        bind_count += 1;
        bind_count += 1;
        if self.created_at.is_some() {
            bind_count += 1;
        }
        if self.updated_at.is_some() {
            bind_count += 1;
        }
        let binds = (1..=bind_count)
            .map(|i| format!("${}", i))
            .collect::<Vec<_>>()
            .join(", ");
        let statement = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            "users", target_columns, binds
        );
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![];
        params.push(&self.email);
        params.push(&self.name);
        params.push(&self.status);
        if self.created_at.is_some() {
            params.push(&self.created_at);
        }
        if self.updated_at.is_some() {
            params.push(&self.updated_at);
        }
        let row = client.query_one(statement.as_str(), &params[..]).await?;
        Ok(row.into())
    }
}
impl From<tokio_postgres::row::Row> for User {
    fn from(row: tokio_postgres::row::Row) -> Self {
        Self {
            id: row.get(0usize),
            email: row.get(1usize),
            name: row.get(2usize),
            status: row.get(3usize),
            created_at: row.get(4usize),
            updated_at: row.get(5usize),
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct UserBuilder {
    pub from: String,
    pub table_name_as: Option<String>,
    pub filters: Vec<Filter>,
    pub preload: bool,
    pub orders: Vec<OrderItem>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}
impl UserBuilder {
    pub fn id(&self) -> UserBuilder_id {
        UserBuilder_id {
            builder: self.clone(),
        }
    }
    pub fn email(&self) -> UserBuilder_email {
        UserBuilder_email {
            builder: self.clone(),
        }
    }
    pub fn name(&self) -> UserBuilder_name {
        UserBuilder_name {
            builder: self.clone(),
        }
    }
    pub fn status(&self) -> UserBuilder_status {
        UserBuilder_status {
            builder: self.clone(),
        }
    }
    pub fn created_at(&self) -> UserBuilder_created_at {
        UserBuilder_created_at {
            builder: self.clone(),
        }
    }
    pub fn updated_at(&self) -> UserBuilder_updated_at {
        UserBuilder_updated_at {
            builder: self.clone(),
        }
    }
    pub fn limit(&self, value: usize) -> Self {
        Self {
            limit: Some(value),
            ..self.clone()
        }
    }
    pub fn offset(&self, value: usize) -> Self {
        Self {
            offset: Some(value),
            ..self.clone()
        }
    }
    pub fn preload(&self) -> Self {
        Self {
            preload: true,
            ..self.clone()
        }
    }
    pub async fn count(&self, client: &tokio_postgres::Client) -> anyhow::Result<i64> {
        let (sql, params) = BuilderTrait::count(self);
        let row = client.query_one(sql.as_str(), &params).await?;
        let x: i64 = row.get(0);
        Ok(x)
    }
    pub async fn first(&self, client: &tokio_postgres::Client) -> anyhow::Result<User> {
        let params = self.select_params();
        let row = client
            .query_one(self.select_sql().as_str(), &params[..])
            .await?;
        let x: User = User::from(row);
        Ok(x)
    }
    #[async_recursion]
    pub async fn load(&self, client: &tokio_postgres::Client) -> anyhow::Result<Vec<User>> {
        let params = self.select_params();
        let rows = client
            .query(self.select_sql().as_str(), &params[..])
            .await?;
        let mut result: Vec<User> = rows.into_iter().map(|row| User::from(row)).collect();
        Ok(result)
    }
}
impl BuilderTrait for UserBuilder {
    fn select(&self) -> String {
        "users".to_string()
    }
    fn from(&self) -> String {
        let mut result: Vec<String> = vec![self.from.clone()];
        self.join(&mut result);
        result.join(" ")
    }
    fn join(&self, join_parts: &mut Vec<String>) {}
    fn filters(&self) -> Vec<&Filter> {
        let mut result: Vec<&Filter> = self.filters.iter().collect();
        result
    }
    fn order(&self) -> &Vec<OrderItem> {
        &self.orders
    }
    fn limit(&self) -> Option<usize> {
        self.limit
    }
    fn offset(&self) -> Option<usize> {
        self.offset
    }
}
#[allow(non_camel_case_types)]
pub struct UserBuilder_id {
    pub builder: UserBuilder,
}
impl UserBuilder_id {
    pub fn eq(&self, value: i32) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(id).to_string(),
            values: vec![Box::new(value)],
            operator: "=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn gt(&self, value: i32) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(id).to_string(),
            values: vec![Box::new(value)],
            operator: ">".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn lt(&self, value: i32) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(id).to_string(),
            values: vec![Box::new(value)],
            operator: "<".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn gte(&self, value: i32) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(id).to_string(),
            values: vec![Box::new(value)],
            operator: ">=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn lte(&self, value: i32) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(id).to_string(),
            values: vec![Box::new(value)],
            operator: "<=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn not_eq(&self, value: i32) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(id).to_string(),
            values: vec![Box::new(value)],
            operator: "<>".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn is_null(&self) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(id).to_string(),
            values: vec![],
            operator: "IS NULL".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn is_not_null(&self) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(id).to_string(),
            values: vec![],
            operator: "IS NOT NULL".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn between(&self, from: i32, to: i32) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(id).to_string(),
            values: vec![Box::new(from), Box::new(to)],
            operator: "BETWEEN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn eq_any(&self, values: Vec<i32>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(id).to_string(),
            values: vs,
            operator: "IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn r#in(&self, values: Vec<i32>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(id).to_string(),
            values: vs,
            operator: "IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn not_in(&self, values: Vec<i32>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(id).to_string(),
            values: vs,
            operator: "NOT IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
}
#[allow(non_camel_case_types)]
pub struct UserBuilder_email {
    pub builder: UserBuilder,
}
impl UserBuilder_email {
    pub fn eq(&self, value: String) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(email).to_string(),
            values: vec![Box::new(value)],
            operator: "=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn gt(&self, value: String) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(email).to_string(),
            values: vec![Box::new(value)],
            operator: ">".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn lt(&self, value: String) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(email).to_string(),
            values: vec![Box::new(value)],
            operator: "<".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn gte(&self, value: String) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(email).to_string(),
            values: vec![Box::new(value)],
            operator: ">=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn lte(&self, value: String) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(email).to_string(),
            values: vec![Box::new(value)],
            operator: "<=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn not_eq(&self, value: String) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(email).to_string(),
            values: vec![Box::new(value)],
            operator: "<>".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn is_null(&self) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(email).to_string(),
            values: vec![],
            operator: "IS NULL".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn is_not_null(&self) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(email).to_string(),
            values: vec![],
            operator: "IS NOT NULL".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn between(&self, from: String, to: String) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(email).to_string(),
            values: vec![Box::new(from), Box::new(to)],
            operator: "BETWEEN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn eq_any(&self, values: Vec<String>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(email).to_string(),
            values: vs,
            operator: "IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn r#in(&self, values: Vec<String>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(email).to_string(),
            values: vs,
            operator: "IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn not_in(&self, values: Vec<String>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(email).to_string(),
            values: vs,
            operator: "NOT IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
}
#[allow(non_camel_case_types)]
pub struct UserBuilder_name {
    pub builder: UserBuilder,
}
impl UserBuilder_name {
    pub fn eq(&self, value: String) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(name).to_string(),
            values: vec![Box::new(value)],
            operator: "=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn gt(&self, value: String) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(name).to_string(),
            values: vec![Box::new(value)],
            operator: ">".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn lt(&self, value: String) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(name).to_string(),
            values: vec![Box::new(value)],
            operator: "<".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn gte(&self, value: String) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(name).to_string(),
            values: vec![Box::new(value)],
            operator: ">=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn lte(&self, value: String) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(name).to_string(),
            values: vec![Box::new(value)],
            operator: "<=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn not_eq(&self, value: String) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(name).to_string(),
            values: vec![Box::new(value)],
            operator: "<>".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn is_null(&self) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(name).to_string(),
            values: vec![],
            operator: "IS NULL".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn is_not_null(&self) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(name).to_string(),
            values: vec![],
            operator: "IS NOT NULL".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn between(&self, from: String, to: String) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(name).to_string(),
            values: vec![Box::new(from), Box::new(to)],
            operator: "BETWEEN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn eq_any(&self, values: Vec<String>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(name).to_string(),
            values: vs,
            operator: "IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn r#in(&self, values: Vec<String>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(name).to_string(),
            values: vs,
            operator: "IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn not_in(&self, values: Vec<String>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(name).to_string(),
            values: vs,
            operator: "NOT IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
}
#[allow(non_camel_case_types)]
pub struct UserBuilder_status {
    pub builder: UserBuilder,
}
impl UserBuilder_status {
    pub fn eq(&self, value: UserStatus) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(status).to_string(),
            values: vec![Box::new(value)],
            operator: "=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn gt(&self, value: UserStatus) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(status).to_string(),
            values: vec![Box::new(value)],
            operator: ">".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn lt(&self, value: UserStatus) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(status).to_string(),
            values: vec![Box::new(value)],
            operator: "<".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn gte(&self, value: UserStatus) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(status).to_string(),
            values: vec![Box::new(value)],
            operator: ">=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn lte(&self, value: UserStatus) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(status).to_string(),
            values: vec![Box::new(value)],
            operator: "<=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn not_eq(&self, value: UserStatus) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(status).to_string(),
            values: vec![Box::new(value)],
            operator: "<>".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn is_null(&self) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(status).to_string(),
            values: vec![],
            operator: "IS NULL".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn is_not_null(&self) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(status).to_string(),
            values: vec![],
            operator: "IS NOT NULL".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn between(&self, from: UserStatus, to: UserStatus) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(status).to_string(),
            values: vec![Box::new(from), Box::new(to)],
            operator: "BETWEEN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn eq_any(&self, values: Vec<UserStatus>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(status).to_string(),
            values: vs,
            operator: "IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn r#in(&self, values: Vec<UserStatus>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(status).to_string(),
            values: vs,
            operator: "IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn not_in(&self, values: Vec<UserStatus>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(status).to_string(),
            values: vs,
            operator: "NOT IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
}
#[allow(non_camel_case_types)]
pub struct UserBuilder_created_at {
    pub builder: UserBuilder,
}
impl UserBuilder_created_at {
    pub fn eq(&self, value: chrono::NaiveDateTime) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(created_at).to_string(),
            values: vec![Box::new(value)],
            operator: "=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn gt(&self, value: chrono::NaiveDateTime) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(created_at).to_string(),
            values: vec![Box::new(value)],
            operator: ">".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn lt(&self, value: chrono::NaiveDateTime) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(created_at).to_string(),
            values: vec![Box::new(value)],
            operator: "<".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn gte(&self, value: chrono::NaiveDateTime) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(created_at).to_string(),
            values: vec![Box::new(value)],
            operator: ">=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn lte(&self, value: chrono::NaiveDateTime) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(created_at).to_string(),
            values: vec![Box::new(value)],
            operator: "<=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn not_eq(&self, value: chrono::NaiveDateTime) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(created_at).to_string(),
            values: vec![Box::new(value)],
            operator: "<>".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn is_null(&self) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(created_at).to_string(),
            values: vec![],
            operator: "IS NULL".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn is_not_null(&self) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(created_at).to_string(),
            values: vec![],
            operator: "IS NOT NULL".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn between(&self, from: chrono::NaiveDateTime, to: chrono::NaiveDateTime) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(created_at).to_string(),
            values: vec![Box::new(from), Box::new(to)],
            operator: "BETWEEN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn eq_any(&self, values: Vec<chrono::NaiveDateTime>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(created_at).to_string(),
            values: vs,
            operator: "IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn r#in(&self, values: Vec<chrono::NaiveDateTime>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(created_at).to_string(),
            values: vs,
            operator: "IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn not_in(&self, values: Vec<chrono::NaiveDateTime>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(created_at).to_string(),
            values: vs,
            operator: "NOT IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
}
#[allow(non_camel_case_types)]
pub struct UserBuilder_updated_at {
    pub builder: UserBuilder,
}
impl UserBuilder_updated_at {
    pub fn eq(&self, value: chrono::NaiveDateTime) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(updated_at).to_string(),
            values: vec![Box::new(value)],
            operator: "=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn gt(&self, value: chrono::NaiveDateTime) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(updated_at).to_string(),
            values: vec![Box::new(value)],
            operator: ">".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn lt(&self, value: chrono::NaiveDateTime) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(updated_at).to_string(),
            values: vec![Box::new(value)],
            operator: "<".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn gte(&self, value: chrono::NaiveDateTime) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(updated_at).to_string(),
            values: vec![Box::new(value)],
            operator: ">=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn lte(&self, value: chrono::NaiveDateTime) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(updated_at).to_string(),
            values: vec![Box::new(value)],
            operator: "<=".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn not_eq(&self, value: chrono::NaiveDateTime) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(updated_at).to_string(),
            values: vec![Box::new(value)],
            operator: "<>".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn is_null(&self) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(updated_at).to_string(),
            values: vec![],
            operator: "IS NULL".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn is_not_null(&self) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(updated_at).to_string(),
            values: vec![],
            operator: "IS NOT NULL".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn between(&self, from: chrono::NaiveDateTime, to: chrono::NaiveDateTime) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(updated_at).to_string(),
            values: vec![Box::new(from), Box::new(to)],
            operator: "BETWEEN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn eq_any(&self, values: Vec<chrono::NaiveDateTime>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(updated_at).to_string(),
            values: vs,
            operator: "IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn r#in(&self, values: Vec<chrono::NaiveDateTime>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(updated_at).to_string(),
            values: vs,
            operator: "IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn not_in(&self, values: Vec<chrono::NaiveDateTime>) -> UserBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: self
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            name: stringify!(updated_at).to_string(),
            values: vs,
            operator: "NOT IN".to_string(),
        });
        UserBuilder {
            filters,
            ..self.builder.clone()
        }
    }
}
impl UserBuilder {
    pub fn order(&self) -> UserOrderBuilder {
        UserOrderBuilder {
            builder: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct UserOrderBuilder {
    pub builder: UserBuilder,
}
impl UserOrderBuilder {
    pub fn id(&self) -> UserOrderAscOrDesc {
        UserOrderAscOrDesc {
            field: "id",
            order_builder: self.clone(),
        }
    }
    pub fn email(&self) -> UserOrderAscOrDesc {
        UserOrderAscOrDesc {
            field: "email",
            order_builder: self.clone(),
        }
    }
    pub fn name(&self) -> UserOrderAscOrDesc {
        UserOrderAscOrDesc {
            field: "name",
            order_builder: self.clone(),
        }
    }
    pub fn status(&self) -> UserOrderAscOrDesc {
        UserOrderAscOrDesc {
            field: "status",
            order_builder: self.clone(),
        }
    }
    pub fn created_at(&self) -> UserOrderAscOrDesc {
        UserOrderAscOrDesc {
            field: "created_at",
            order_builder: self.clone(),
        }
    }
    pub fn updated_at(&self) -> UserOrderAscOrDesc {
        UserOrderAscOrDesc {
            field: "updated_at",
            order_builder: self.clone(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct UserOrderAscOrDesc {
    pub field: &'static str,
    pub order_builder: UserOrderBuilder,
}
impl UserOrderAscOrDesc {
    pub fn asc(&self) -> UserBuilder {
        let mut builder = self.order_builder.builder.clone();
        builder.orders.push(OrderItem {
            table: self
                .order_builder
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            field: self.field,
            asc_or_desc: "ASC",
        });
        builder
    }
    pub fn desc(&self) -> UserBuilder {
        let mut builder = self.order_builder.builder.clone();
        builder.orders.push(OrderItem {
            table: self
                .order_builder
                .builder
                .table_name_as
                .as_ref()
                .unwrap_or(&"users".to_string())
                .to_string(),
            field: self.field,
            asc_or_desc: "DESC",
        });
        builder
    }
}
