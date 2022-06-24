```
cargo new --lib xxx
```

#### Cargo.toml

```toml
[dependencies]
wasm-bindgen = "0.2.81"

[lib]
crate-type = ["cdylib"]
```

#### lib.rs

```rs
use wasm_bindgen::prelude::*;
// 需要包裹上这块宏
#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
```

#### 构建

https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html

直接使用就用 `--target web`，否则需要用 webpack 处理。

```
// 会生成到 pkg 目录
wasm-pack build --target web
```

```
.
├─ README.md
├─ xxx.d.ts
├─ xxx.js
├─ xxx_bg.wasm
└─ xxx_bg.wasm.d.ts
```

xxx.js 本质上有无都可以，你可以直接使用 xxx_bg.wasm 来消费。

但好处在于 xxx.js 是由 wasm-pack 生成的，内部有加载函数和 lib 结构，把一些能力都给做好了，也没过多的无用代码，用起来是非常方便的。

#### 使用

```html
// xxx.js default 是一个初始化函数，反回一个 wasm 的原始 exports，和 wasm
直接使用。
<script type="module">
  import initWasm from "./pkg/xxx.js";
  initWasm().then(({ add }) => {});
</script>

// 或者 // 但这种 import 的 add 是由 xxx.js
自己根据一定推测包裹起来的，所以本质来说他并不是真实 wasm
的函数，而是又做了一层pollyfill，有利有弊吧
<script type="module">
  import initWasm, { add } from "./pkg/xxx.js";
  initWasm().then(() => {
    add(1, 2);
  });
</script>

// 或者自行 Res.arraybuffer，然后同步加载
<script type="module">
  import { initSync } from "./pkg/xxx.js";

  // async
  const byte = Res.arrayBuffer();

  const { add } = initSync(byte);
</script>
```

#### npm 发布/消费


```
wasm-pack build 
wasm-pack publish
```

即可，会根据 Cargo.toml 的信息生成 package.json。

> 默认 wasm-pack build 是生成 webpack 能支持的代码，所以需要在用 webpack 去构建。
> 
> 注意 cdn 情况下加载路径的问题。

https://webpack.docschina.org/configuration/output/#outputpublicpath