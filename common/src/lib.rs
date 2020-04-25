#[cfg_attr(target_arch = "x86_64", macro_use)]
#[cfg(target_arch = "x86_64")]
extern crate diesel;

pub mod dto;
pub mod form;
#[cfg(target_arch = "x86_64")]
pub mod schema;
pub mod types;
pub mod validate;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
