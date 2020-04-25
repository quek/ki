//use std::io::Write;

/*
↓このワーニングを消すために
warning: unused import: `common::types::*`
 --> src/schema.rs:3:9
  |
3 |     use crate::common::types::*;
  |         ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default
*/
fn main() {
    //    let allow = "    #[allow(unused_imports)]";
    //    let schema = std::fs::read_to_string("src/schema.rs").unwrap();
    //    for line in schema.lines() {
    //        if line == allow {
    //            return;
    //        }
    //    }
    //    let mut writer = std::io::BufWriter::new(std::fs::File::create("src/schema.rs").unwrap());
    //    for line in schema.lines() {
    //        if line == "    use common::types::*;" {
    //            writeln!(writer, "{}", allow).unwrap();
    //        }
    //        writeln!(writer, "{}", line).unwrap();
    //    }
}
