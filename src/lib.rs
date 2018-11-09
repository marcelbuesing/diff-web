extern crate cfg_if;
extern crate wasm_bindgen;
extern crate difference;
#[macro_use]
extern crate serde_derive;
use difference::{Difference, Changeset};

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}


#[wasm_bindgen]
#[derive(Debug, Deserialize)]
pub struct DiffColors {
    added: ColorDef,
    added_highlighted: ColorDef,
    removed: ColorDef,
    removed_highlighted: ColorDef,
    same: ColorDef,
}


#[wasm_bindgen]
#[derive(Debug, Deserialize)]
/// Color definition
pub struct ColorDef {
    /// Foreground color
    fg: String,
    /// Background color
    bg: String,
}

#[wasm_bindgen]
pub fn diff(text1: &str, text2: &str, diff_colors: &JsValue) -> String {
    let diff_colors: DiffColors = diff_colors.into_serde().unwrap();
    let Changeset { diffs, .. } = Changeset::new(text1, text2, "\n");
    let mut s = String::new();
    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref x) => {
                s.push_str(&format!(" {}", x.replace("\n", "&#13;&#10;")));
            }
            Difference::Add(ref x) => {
                match diffs[i - 1] {
                    Difference::Rem(ref y) => {
                        //t.fg(term::color::GREEN).unwrap();
                        s.push_str(&format!("<span style=\"color:{};background-color:{};\">+ </span>", diff_colors.added.fg, diff_colors.added.bg));
                        let Changeset { diffs, .. } = Changeset::new(y, x, " ");
                        for c in diffs {
                            match c {
                                Difference::Same(ref z) => {
                                   // t.fg(term::color::GREEN).unwrap();
                                    s.push_str(&format!("<span style=\"color:{};background-color:{};\">{}", diff_colors.same.fg, diff_colors.same.bg, z));
                                    s.push_str(&format!("</span> "));
                                }
                                Difference::Add(ref z) => {
                                    //t.fg(term::color::WHITE).unwrap();
                                    //t.bg(term::color::GREEN).unwrap();
                                    s.push_str(&format!("<span style=\"color:{};background-color:{};\">", diff_colors.added_highlighted.fg, diff_colors.added_highlighted.bg));
                                    s.push_str(&format!("{}", z));
                                    s.push_str(&format!("</span> "));
                                }
                                _ => (),
                            }
                        }
                        s.push_str(&format!("&#13;&#10;"));
                    }
                    _ => {
                        //t.fg(term::color::BRIGHT_GREEN).unwrap();
                        s.push_str(&format!("<span style=\"color:{};background-color:{};\">+ {}</span></br>", diff_colors.added.fg, diff_colors.added.bg, x));
                    }
                };
            }
            Difference::Rem(ref x) => {
                //t.fg(term::color::RED).unwrap();
                s.push_str(&format!("<span style=\"color:{};background-color:{};\">- {}</span></br>",  diff_colors.removed.fg, diff_colors.removed.bg, x));
            }
        }
    }
    s
}