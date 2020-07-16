use anyhow::Result;
use arysn::generator::config::Config;
use arysn::generator::define_ar;
use quote::format_ident;

fn main() -> Result<()> {
    define_ar(&Config {
        path: "src/generated/user.rs",
        table_name: "users",
        struct_name: format_ident!("{}", "User"),
        has_many: vec![],
        belongs_to: vec![],
    })?;

    define_ar(&Config {
        path: "src/generated/post.rs",
        table_name: "posts",
        struct_name: format_ident!("{}", "Post"),
        has_many: vec![],
        belongs_to: vec![],
    })?;

    Ok(())
}
