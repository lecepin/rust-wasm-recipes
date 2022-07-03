示例中通过 expect 的方式，错误情况下，如：

```rust
pub fn run() -> Result<(), JsValue> {
  ...
    let body = document.body().expect("document should have a body");
  ...
}
```

```js
document.body.remove();
import initWasm from "./pkg/lp_demo_rust_wasm.js";
initWasm().catch((e) => {
  console.log({ e });
});
```

并没有获得预期的有效信息：
![image](https://user-images.githubusercontent.com/11046969/177038094-f5538a12-f44c-4aed-9cf9-bf97d43733d8.png)
