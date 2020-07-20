use crate::errors::ServiceError;
use crate::generated::post::{Post, PostStatus};
use crate::thread_data::ThreadData;
use actix_web::{web, HttpResponse};

pub async fn index(data: web::Data<ThreadData>) -> Result<HttpResponse, ServiceError> {
    let conn = &data.dpool.get().await.unwrap();
    let result: Vec<Post> = Post::select()
        .status()
        .eq(PostStatus::Published)
        .order("posts.published_at desc, posts.id desc")
        .load(conn)
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(result))
}

pub async fn show(
    params: web::Path<i32>,
    data: web::Data<ThreadData>,
) -> Result<HttpResponse, ServiceError> {
    let conn = &data.dpool.get().await.unwrap();
    let id = params.into_inner();
    let result: Post = Post::select()
        .status()
        .eq(PostStatus::Published)
        .id()
        .eq(id)
        .first(conn)
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(result))
}
