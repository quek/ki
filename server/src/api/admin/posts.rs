use crate::common::form::PostForm;
use crate::common::models::{Id, Post, PostQuery, PER_PAGE};
use crate::common::types::PostStatus;
use crate::errors::ServiceError;
use crate::paginate::Paginate;
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
    published_at: Option<chrono::NaiveDateTime>,
}

impl From<PostForm> for PostValues {
    fn from(form: PostForm) -> Self {
        PostValues {
            title: form.title,
            body: form.body,
            status: PostStatus::from_str(&form.status).unwrap(),
            published_at: None,
        }
    }
}

pub async fn index(
    query: web::Query<PostQuery>,
    data: web::Data<ThreadData>,
) -> Result<HttpResponse, ServiceError> {
    let result: (Vec<Post>, i64) = web::block(move || {
        let query = query.into_inner();
        let page = std::cmp::max(query.page, 1);
        let conn: &PgConnection = &data.pool.get().unwrap();
        let result = posts::table
            .order(posts::updated_at.desc())
            .paginate(page)
            .per_page(PER_PAGE)
            .load_and_count::<Post>(conn)?;
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
        match form.validate() {
            Ok(()) => {
                let mut values: PostValues = form.into();
                if values.status == PostStatus::Published {
                    let post: Post = posts::table.filter(posts::id.eq(id)).first(conn)?;
                    if post.published_at.is_none() {
                        values.published_at = Some(chrono::Local::now().naive_local())
                    }
                }

                diesel::update(posts::table.filter(posts::id.eq(id)))
                    .set(values)
                    .execute(conn)?;
                Ok(Ok(()))
            }
            Err(errors) => Ok(Err(errors)),
        }
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
        match form.validate() {
            Ok(()) => {
                let mut values: PostValues = form.into();
                if values.status == PostStatus::Published {
                    values.published_at = Some(chrono::Local::now().naive_local())
                }
                diesel::insert_into(posts::table)
                    .values(values)
                    .execute(conn)?;
                Ok(Ok(()))
            }
            Err(errors) => Ok(Err(errors)),
        }
    })
    .await?;
    Ok(HttpResponse::Ok().json(result))
}
