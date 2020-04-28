use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "prismjs")]
extern "C" {
    pub type Language;
    pub static languages: Language;

    #[wasm_bindgen(method, structural, indexing_getter)]
    pub fn get(this: &Language, prop: String) -> Option<Language>;

    pub fn highlight(code: String, grammar: Language, lang: String) -> String;
}
