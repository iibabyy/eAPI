#![allow(clippy::non_std_lazy_statics)]

use lazy_static::lazy_static;

lazy_static! {
    pub static ref REFRESH_TOKEN: &'static str = "refresh_token";
}
