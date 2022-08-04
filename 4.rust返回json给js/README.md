### Ref
- [Serializing and Deserializing Arbitrary Data Into and From JsValue with Serde ](https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html)

---

```toml
[dependencies]
serde_json = "1.0.83"
serde = { version = "1.0.138", features = ["derive"] }
wasm-bindgen = { version = "0.2.81", features = ["serde-serialize"] }
```

```rust
#[cfg(feature = "serde-serialize")]
pub fn from_serde<T>(t: &T) -> serde_json::Result<JsValue>
where
    T: serde::ser::Serialize + ?Sized,
{
    let s = serde_json::to_string(t)?;
    unsafe { Ok(JsValue::_new(__wbindgen_json_parse(s.as_ptr(), s.len()))) }
}
```
