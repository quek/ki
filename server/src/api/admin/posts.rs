use crate::common::dto::{Id, Post};
use crate::common::form::PostForm;
use crate::common::types::PostStatus;
use crate::errors::ServiceError;
use crate::schema;
use crate::thread_data::ThreadData;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use diesel::AsChangeset;
use std::str::FromStr;

pub async fn index(data: web::Data<ThreadData>) -> Result<HttpResponse, ServiceError> {
    let result: Vec<Post> = web::block(move || {
        use schema::posts;
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
        use schema::posts;
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
        use schema::posts;
        let conn: &PgConnection = &data.pool.get().unwrap();
        let id = params.into_inner();
        let form = form.into_inner();

        #[derive(AsChangeset)]
        #[table_name = "posts"]
        struct Values {
            title: String,
            body: String,
            status: PostStatus,
        }

        let values = Values {
            title: form.title,
            body: form.body,
            status: PostStatus::from_str(&form.status).unwrap(),
        };

        diesel::update(posts::table.filter(posts::id.eq(id)))
            .set(values)
            .execute(conn)?;
        Ok(())
    })
    .await?;
    Ok(HttpResponse::Ok().json(result))
}
