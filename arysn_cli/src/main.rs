use arysn::generator::config::Config;
use arysn::generator::define_ar;

fn main() {
    define_ar(&Config {
        path: "../arysn_cli/generated/user.rs",
        table_name: "users",
        struct_name: "User",
        has_many: vec![],
        belongs_to: vec![],
    })
    .unwrap();

    define_ar(&Config {
        path: "../arysn_cli/generated/post.rs",
        table_name: "posts",
        struct_name: "Post",
        has_many: vec![],
        belongs_to: vec![],
    })
    .unwrap();
}
