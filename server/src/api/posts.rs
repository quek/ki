use crate::common::models::{Id, Post};
use crate::common::types::PostStatus;
use crate::errors::ServiceError;
use crate::schema;
use crate::thread_data::ThreadData;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn index(data: web::Data<ThreadData>) -> Result<HttpResponse, ServiceError> {
    let result: Vec<Post> = web::block(move || {
        use schema::posts;
        let conn: &PgConnection = &data.pool.get().unwrap();
        let result = posts::table
            .filter(posts::status.eq(PostStatus::Published))
            .order(posts::published_at.desc())
            .order(posts::id.desc())
            .limit(20)
            .load(conn)?;
        Ok(result)
    })
    .await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn show(
    params: web::Path<Id>,
    data: web::Data<ThreadData>,
) -> Result<HttpResponse, ServiceError> {
    let result: Post = web::block(move || {
        use schema::posts;
        let conn: &PgConnection = &data.pool.get().unwrap();
        let id = params.into_inner();
        let result = posts::table
            .filter(posts::status.eq(PostStatus::Published))
            .filter(posts::id.eq(id))
            .first(conn)?;
        Ok(result)
    })
    .await?;
    Ok(HttpResponse::Ok().json(result))
}
