use crate::common::dto::{Id, Post};
use crate::common::form::PostForm;
use crate::common::types::PostStatus;
use crate::errors::ServiceError;
use crate::schema::posts;
use crate::thread_data::ThreadData;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use diesel::{AsChangeset, Insertable};
use std::str::FromStr;

#[derive(AsChangeset, Insertable)]
#[table_name = "posts"]
struct PostValues {
    title: String,
    body: String,
    status: PostStatus,
}

impl From<PostForm> for PostValues {
    fn from(form: PostForm) -> Self {
        PostValues {
            title: form.title,
            body: form.body,
            status: PostStatus::from_str(&form.status).unwrap(),
        }
    }
}

pub async fn index(data: web::Data<ThreadData>) -> Result<HttpResponse, ServiceError> {
    let result: Vec<Post> = web::block(move || {
        let conn: &PgConnection = &data.pool.get().unwrap();
        let result = posts::table
            .order(posts::created_at.desc())
            .limit(20)
            .load(conn)?;
        Ok(result)
    })
    .await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn edit(
    params: web::Path<Id>,
    data: web::Data<ThreadData>,
) -> Result<HttpResponse, ServiceError> {
    let result: Post = web::block(move || {
        let conn: &PgConnection = &data.pool.get().unwrap();
        let id = params.into_inner();
        let result = posts::table.filter(posts::id.eq(id)).first(conn)?;
        Ok(result)
    })
    .await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn update(
    params: web::Path<Id>,
    form: web::Json<PostForm>,
    data: web::Data<ThreadData>,
) -> Result<HttpResponse, ServiceError> {
    let result = web::block(move || {
        let conn: &PgConnection = &data.pool.get().unwrap();
        let id = params.into_inner();
        let form = form.into_inner();

        let values: PostValues = form.into();
        diesel::update(posts::table.filter(posts::id.eq(id)))
            .set(values)
            .execute(conn)?;
        Ok(())
    })
    .await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn create(
    form: web::Json<PostForm>,
    data: web::Data<ThreadData>,
) -> Result<HttpResponse, ServiceError> {
    let result = web::block(move || {
        let conn: &PgConnection = &data.pool.get().unwrap();
        let form = form.into_inner();

        let values: PostValues = form.into();
        diesel::insert_into(posts::table)
            .values(values)
            .execute(conn)?;
        Ok(())
    })
    .await?;
    Ok(HttpResponse::Ok().json(result))
}
