// これはスレッドごと
// 全スレッド共通の場合は https://actix.rs/docs/application/#shared-mutable-state
pub struct ThreadData {
    pub dpool: deadpool_postgres::Pool,
}
