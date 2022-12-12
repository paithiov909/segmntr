use extendr_api::prelude::*;

use std::fs::File;
use vaporetto::{Model, Predictor, Sentence};

/// Internal wrapper of vaporetto
/// @keywords internal
#[extendr]
fn vaporetto(x: Vec<String>, model: String) -> Robj {

    let f = zstd::Decoder::new(File::open(model).unwrap()).unwrap();
    let model = Model::read(f).unwrap();
    let predictor = Predictor::new(model, true).unwrap();

    let capacity = x.len();
    let mut tokens: Vec<_> = Vec::with_capacity(capacity);

    let mut s = Sentence::default();

    for (_, text) in x.iter().enumerate() {
      let mut v = String::new();
      s.update_raw(text.as_str()).unwrap();
      predictor.predict(&mut s);
      s.fill_tags();
      s.write_tokenized_text(&mut v);
      tokens.push(r!(v));
    }
    return r!(tokens);
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod segmntr;
    fn vaporetto;
}
