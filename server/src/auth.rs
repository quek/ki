use crate::errors::ServiceError;
use crate::thread_data::ThreadData;
use actix_identity::Identity;
use actix_web::client::Client;
use actix_web::web;
use actix_web::HttpResponse;
use common::dto::User;
use common::types::UserStatus;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::Deserialize;
use std::collections::HashMap;
use std::env::var;
use url::Url;

lazy_static::lazy_static! {
    static ref REDIRECT_URL: String = std::env::var("REDIRECT_URL").expect("REDIRECT_URL must be set!");
}

pub async fn login() -> Result<HttpResponse, ServiceError> {
    let endpoint = "https://accounts.google.com/o/oauth2/v2/auth";
    let client_id = var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set!");
    let response_type = "code";
    let scope = "email";
    let access_type = "online";
    let state = "TODO Specifies any string value";

    let mut url = Url::parse(endpoint).unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", &client_id)
        .append_pair("redirect_uri", &REDIRECT_URL)
        .append_pair("response_type", response_type)
        .append_pair("scope", scope)
        .append_pair("access_type", access_type)
        .append_pair("state", state);
    Ok(HttpResponse::Ok().json(url.as_str()))
}

pub async fn login_code(
    query: web::Query<HashMap<String, String>>,
    identity: Identity,
    data: web::Data<ThreadData>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(code) = query.get("code") {
        let endpoint = "https://oauth2.googleapis.com/token";
        let client_id = var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set!");
        let client_secret = var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set!");
        let grant_type = "authorization_code";

        let mut request = std::collections::HashMap::new();
        request.insert("client_id".to_string(), client_id);
        request.insert("client_secret".to_string(), client_secret);
        request.insert("code".to_string(), code.to_string());
        request.insert("grant_type".to_string(), grant_type.to_string());
        request.insert("redirect_uri".to_string(), REDIRECT_URL.to_string());

        let url = Url::parse(endpoint).unwrap();
        let client = Client::default();
        let response = client.post(url.as_str()).send_json(&request).await;
        // println!("login_code response {:?}", &response);

        #[derive(Debug, Deserialize)]
        struct Response {
            access_token: String,
            expires_in: i32,
            token_type: String,
            scope: String,
            // ↓は access_type を offline にした時だけある
            refresh_token: Option<String>,
        }
        let response: Response = response.unwrap().json().await.unwrap();
        // println!("login_code r {:?}", &response);
        let access_token = response.access_token;

        let response = client
            .get("https://www.googleapis.com/oauth2/v1/userinfo")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await;
        // println!("userinfo response {:?}", &response);

        #[derive(Debug, Deserialize)]
        struct UserInfo {
            id: String,
            // name: String,
            email: String,
            // locale: String,
            // nfamily_name: String,
            // link: String,
            // given_name: String,
            // picture: String,
        }
        let user_info: UserInfo = response.unwrap().json().await.unwrap();
        println!("user_info {:?}", &user_info);
        // let x = response.unwrap().body().await.unwrap();
        // println!("user_info {:?}", &x);

        let user = web::block(move || {
            use common::schema::users;
            let conn: &PgConnection = &data.pool.get().unwrap();
            let user: User = users::table
                .filter(users::email.eq(user_info.email))
                .filter(users::status.eq(UserStatus::Active))
                .first(conn)?;
            Ok(user)
        })
        .await?;

        identity.remember(serde_json::to_string(&user).unwrap());
        Ok(HttpResponse::Found().header("Location", "/admin").finish())
    } else {
        Err(ServiceError::Unauthorized)
    }
}
