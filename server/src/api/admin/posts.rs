use crate::common::form::PostForm;
use crate::common::models::{Id, PostQuery, PER_PAGE};
use crate::errors::ServiceError;
use crate::generated::post::{Post, PostNew, PostStatus};
use crate::thread_data::ThreadData;
use actix_web::{web, HttpResponse};
use std::str::FromStr;

pub async fn index(
    query: web::Query<PostQuery>,
    data: web::Data<ThreadData>,
) -> Result<HttpResponse, ServiceError> {
    let query = query.into_inner();
    let page = std::cmp::max(query.page, 1);
    let conn = &data.dpool.get().await.unwrap();
    let sql = Post::select().order().updated_at().desc();
    let count = sql.count(conn).await?;
    let posts = sql
        .limit(PER_PAGE)
        .offset(((page - 1) * PER_PAGE) as usize)
        .load(conn)
        .await?;

    let result: (Vec<Post>, i64) = (posts, count);
    Ok(HttpResponse::Ok().json(result))
}

pub async fn edit(
    params: web::Path<Id>,
    data: web::Data<ThreadData>,
) -> Result<HttpResponse, ServiceError> {
    let conn = &data.dpool.get().await.unwrap();
    let id = params.into_inner();
    let result = Post::select().id().eq(id).first(conn).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn update(
    params: web::Path<Id>,
    form: web::Json<PostForm>,
    data: web::Data<ThreadData>,
) -> Result<HttpResponse, ServiceError> {
    let conn = &data.dpool.get().await.unwrap();
    let id = params.into_inner();
    let form = form.into_inner();
    let result = match form.validate() {
        Ok(_) => {
            let mut post = Post::select().id().eq(id).first(conn).await?;
            post.title = form.title;
            post.body = form.body;
            post.status = PostStatus::from_str(&form.status).unwrap();
            if post.status == PostStatus::Published {
                if post.published_at.is_none() {
                    post.published_at = Some(chrono::Local::now().naive_local())
                }
            }
            post.update(conn).await?;

            Ok(())
        }
        Err(errors) => Err(errors),
    };
    Ok(HttpResponse::Ok().json(result))
}

pub async fn create(
    form: web::Json<PostForm>,
    data: web::Data<ThreadData>,
) -> Result<HttpResponse, ServiceError> {
    let conn = &data.dpool.get().await.unwrap();
    let form = form.into_inner();
    let result = match form.validate() {
        Ok(_) => {
            let status = PostStatus::from_str(&form.status).unwrap();
            let published_at = if status == PostStatus::Published {
                Some(chrono::Local::now().naive_local())
            } else {
                None
            };
            let post = PostNew {
                id: None,
                title: form.title,
                body: form.body,
                status,
                published_at,
                created_at: None,
                updated_at: None,
            };
            let _post: Post = post.insert(conn).await?;
            Ok(())
        }
        Err(errors) => Err(errors),
    };
    Ok(HttpResponse::Ok().json(result))
}
