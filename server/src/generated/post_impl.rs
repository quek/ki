use super::post::*;
use arysn::prelude::*;
use async_recursion::async_recursion;
impl Post {
    pub fn select() -> PostBuilder {
        PostBuilder {
            from: "posts".to_string(),
            ..PostBuilder::default()
        }
    }
    pub async fn delete(&self, client: &tokio_postgres::Client) -> anyhow::Result<()> {
        client
            .execute("DELETE FROM posts WHERE id = $1", &[&self.id])
            .await?;
        Ok(())
    }
    pub async fn update(&self, client: &tokio_postgres::Client) -> anyhow::Result<()> {
        client . execute ( "UPDATE posts SET title = $1, body = $2, status = $3, published_at = $4, created_at = $5, updated_at = $6 WHERE id = $7" , & [ & self . title , & self . body , & self . status , & self . published_at , & self . created_at , & self . updated_at , & self . id ] ) . await ? ;
        Ok(())
    }
}
impl PostNew {
    pub async fn insert(&self, client: &tokio_postgres::Client) -> anyhow::Result<Post> {
        let mut target_columns: Vec<&str> = vec![];
        target_columns.push(stringify!(title));
        target_columns.push(stringify!(body));
        target_columns.push(stringify!(status));
        target_columns.push(stringify!(published_at));
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
            "posts", target_columns, binds
        );
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![];
        params.push(&self.title);
        params.push(&self.body);
        params.push(&self.status);
        params.push(&self.published_at);
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
impl From<tokio_postgres::row::Row> for Post {
    fn from(row: tokio_postgres::row::Row) -> Self {
        Self {
            id: row.get(0usize),
            title: row.get(1usize),
            body: row.get(2usize),
            status: row.get(3usize),
            published_at: row.get(4usize),
            created_at: row.get(5usize),
            updated_at: row.get(6usize),
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct PostBuilder {
    pub from: String,
    pub filters: Vec<Filter>,
    pub preload: bool,
    pub order: String,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}
impl PostBuilder {
    pub fn id(&self) -> PostBuilder_id {
        PostBuilder_id {
            builder: self.clone(),
        }
    }
    pub fn title(&self) -> PostBuilder_title {
        PostBuilder_title {
            builder: self.clone(),
        }
    }
    pub fn body(&self) -> PostBuilder_body {
        PostBuilder_body {
            builder: self.clone(),
        }
    }
    pub fn status(&self) -> PostBuilder_status {
        PostBuilder_status {
            builder: self.clone(),
        }
    }
    pub fn published_at(&self) -> PostBuilder_published_at {
        PostBuilder_published_at {
            builder: self.clone(),
        }
    }
    pub fn created_at(&self) -> PostBuilder_created_at {
        PostBuilder_created_at {
            builder: self.clone(),
        }
    }
    pub fn updated_at(&self) -> PostBuilder_updated_at {
        PostBuilder_updated_at {
            builder: self.clone(),
        }
    }
    pub fn order<T: AsRef<str>>(&self, value: T) -> Self {
        Self {
            order: value.as_ref().to_string(),
            ..self.clone()
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
    pub async fn first(&self, client: &tokio_postgres::Client) -> anyhow::Result<Post> {
        let params = self.select_params();
        let row = client
            .query_one(self.select_sql().as_str(), &params[..])
            .await?;
        let x: Post = Post::from(row);
        Ok(x)
    }
    #[async_recursion]
    pub async fn load(&self, client: &tokio_postgres::Client) -> anyhow::Result<Vec<Post>> {
        let params = self.select_params();
        let rows = client
            .query(self.select_sql().as_str(), &params[..])
            .await?;
        let mut result: Vec<Post> = rows.into_iter().map(|row| Post::from(row)).collect();
        Ok(result)
    }
}
impl BuilderTrait for PostBuilder {
    fn select(&self) -> String {
        "posts".to_string()
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
    fn order_part(&self) -> String {
        self.order.clone()
    }
    fn limit(&self) -> Option<usize> {
        self.limit
    }
    fn offset(&self) -> Option<usize> {
        self.offset
    }
}
#[allow(non_camel_case_types)]
pub struct PostBuilder_id {
    pub builder: PostBuilder,
}
impl PostBuilder_id {
    pub fn eq(&self, value: i32) -> PostBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: "posts".to_string(),
            name: stringify!(id).to_string(),
            values: vec![Box::new(value)],
            operator: "=".to_string(),
        });
        PostBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn eq_any(&self, values: Vec<i32>) -> PostBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: "posts".to_string(),
            name: stringify!(id).to_string(),
            values: vs,
            operator: "in".to_string(),
        });
        PostBuilder {
            filters,
            ..self.builder.clone()
        }
    }
}
#[allow(non_camel_case_types)]
pub struct PostBuilder_title {
    pub builder: PostBuilder,
}
impl PostBuilder_title {
    pub fn eq(&self, value: String) -> PostBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: "posts".to_string(),
            name: stringify!(title).to_string(),
            values: vec![Box::new(value)],
            operator: "=".to_string(),
        });
        PostBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn eq_any(&self, values: Vec<String>) -> PostBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: "posts".to_string(),
            name: stringify!(title).to_string(),
            values: vs,
            operator: "in".to_string(),
        });
        PostBuilder {
            filters,
            ..self.builder.clone()
        }
    }
}
#[allow(non_camel_case_types)]
pub struct PostBuilder_body {
    pub builder: PostBuilder,
}
impl PostBuilder_body {
    pub fn eq(&self, value: String) -> PostBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: "posts".to_string(),
            name: stringify!(body).to_string(),
            values: vec![Box::new(value)],
            operator: "=".to_string(),
        });
        PostBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn eq_any(&self, values: Vec<String>) -> PostBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: "posts".to_string(),
            name: stringify!(body).to_string(),
            values: vs,
            operator: "in".to_string(),
        });
        PostBuilder {
            filters,
            ..self.builder.clone()
        }
    }
}
#[allow(non_camel_case_types)]
pub struct PostBuilder_status {
    pub builder: PostBuilder,
}
impl PostBuilder_status {
    pub fn eq(&self, value: PostStatus) -> PostBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: "posts".to_string(),
            name: stringify!(status).to_string(),
            values: vec![Box::new(value)],
            operator: "=".to_string(),
        });
        PostBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn eq_any(&self, values: Vec<PostStatus>) -> PostBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: "posts".to_string(),
            name: stringify!(status).to_string(),
            values: vs,
            operator: "in".to_string(),
        });
        PostBuilder {
            filters,
            ..self.builder.clone()
        }
    }
}
#[allow(non_camel_case_types)]
pub struct PostBuilder_published_at {
    pub builder: PostBuilder,
}
impl PostBuilder_published_at {
    pub fn eq(&self, value: chrono::NaiveDateTime) -> PostBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: "posts".to_string(),
            name: stringify!(published_at).to_string(),
            values: vec![Box::new(value)],
            operator: "=".to_string(),
        });
        PostBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn eq_any(&self, values: Vec<chrono::NaiveDateTime>) -> PostBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: "posts".to_string(),
            name: stringify!(published_at).to_string(),
            values: vs,
            operator: "in".to_string(),
        });
        PostBuilder {
            filters,
            ..self.builder.clone()
        }
    }
}
#[allow(non_camel_case_types)]
pub struct PostBuilder_created_at {
    pub builder: PostBuilder,
}
impl PostBuilder_created_at {
    pub fn eq(&self, value: chrono::NaiveDateTime) -> PostBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: "posts".to_string(),
            name: stringify!(created_at).to_string(),
            values: vec![Box::new(value)],
            operator: "=".to_string(),
        });
        PostBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn eq_any(&self, values: Vec<chrono::NaiveDateTime>) -> PostBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: "posts".to_string(),
            name: stringify!(created_at).to_string(),
            values: vs,
            operator: "in".to_string(),
        });
        PostBuilder {
            filters,
            ..self.builder.clone()
        }
    }
}
#[allow(non_camel_case_types)]
pub struct PostBuilder_updated_at {
    pub builder: PostBuilder,
}
impl PostBuilder_updated_at {
    pub fn eq(&self, value: chrono::NaiveDateTime) -> PostBuilder {
        let mut filters = self.builder.filters.clone();
        filters.push(Filter {
            table: "posts".to_string(),
            name: stringify!(updated_at).to_string(),
            values: vec![Box::new(value)],
            operator: "=".to_string(),
        });
        PostBuilder {
            filters,
            ..self.builder.clone()
        }
    }
    pub fn eq_any(&self, values: Vec<chrono::NaiveDateTime>) -> PostBuilder {
        let mut filters = self.builder.filters.clone();
        let mut vs: Vec<Box<dyn ToSqlValue>> = vec![];
        for v in values {
            vs.push(Box::new(v));
        }
        filters.push(Filter {
            table: "posts".to_string(),
            name: stringify!(updated_at).to_string(),
            values: vs,
            operator: "in".to_string(),
        });
        PostBuilder {
            filters,
            ..self.builder.clone()
        }
    }
}
