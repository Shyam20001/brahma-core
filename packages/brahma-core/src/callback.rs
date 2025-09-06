use napi_derive::napi;
use napi::threadsafe_function::ThreadsafeFunction;
use once_cell::sync::OnceCell;

pub static JS_RESPONSE_CALLBACK: OnceCell<
    ThreadsafeFunction<(String, String, String, String), String>,
> = OnceCell::new();

#[napi]
pub fn register_js_callback(
    callback: ThreadsafeFunction<(String, String, String, String), String>,
) {
    if JS_RESPONSE_CALLBACK.set(callback).is_err() {
        panic!("JS callback already set");
    }
}
