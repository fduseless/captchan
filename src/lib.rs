#![deny(clippy::all)]

use napi::bindgen_prelude::Uint8Array;

mod captcha;

#[macro_use]
extern crate napi_derive;

#[napi]
pub enum Format {
  PNG,
  JPG,
  JPEG,
  WEBP,
}
#[napi]
pub struct Captcha {
  pub text: String,
  pub image: Uint8Array,
}

impl Format {
  fn as_str(&self) -> &'static str {
      match self {
        Format::PNG => "png",
        Format::JPG => "jpg",
        Format::JPEG => "jpeg",
        Format::WEBP => "webp"
      }
  }
}

#[napi]
fn create_image(len: Option<u32>, difficulty: Option<u32>, line: Option<bool>, noise: Option<bool>, format: Option<Format>) -> Captcha {
  let c = captcha::CaptchaBuilder::new();
  let out: captcha::Captcha = c
      .complexity( match difficulty {Some(v) => {v as usize}, None => {5}})
      .length(match len {Some(v) => {v as usize}, None => {4}})
      .line(match line {Some(v) => {v}, None => {true}})
      .noise(match noise {Some(v) => {v}, None => {false}})
      .format(match format {Some(v) => {v.as_str()}, None => {"png"}})
      .build();
  return Captcha {
    text: out.text,
    image: Uint8Array::new(out.image)
  };
}
