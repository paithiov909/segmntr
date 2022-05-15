use extendr_api::prelude::*;

use std::io::{Cursor, Read};
use vaporetto::{Model, Predictor, Sentence};

/// Internal wrapper of vaporetto
/// @keywords internal
#[extendr]
fn vaporetto(x: Vec<String>) -> Robj {
    let mut f = Cursor::new(include_bytes!(env!("VAPORETTO_MODEL_PATH")));
    let mut decoder = ruzstd::StreamingDecoder::new(&mut f).unwrap();
    let mut buff = vec![];
    decoder.read_to_end(&mut buff).unwrap();
    let model = Model::read(&mut buff.as_slice()).unwrap();
    let predictor = Predictor::new(model, false).unwrap();

    let capacity = x.len();
    let mut tokens: Vec<_> = Vec::with_capacity(capacity);

    for (_, text) in x.iter().enumerate() {
      let s = Sentence::from_raw(text.as_str()).unwrap();
      let s = predictor.predict(s);
      let v = s.to_tokenized_vec().unwrap().iter().map(|t| t.surface).collect::<Vec<_>>();
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
