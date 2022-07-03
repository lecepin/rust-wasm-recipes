use js_sys::{Array, Date};
use wasm_bindgen::prelude::*;

// .dyn_ref::<HtmlElement>()
// .unchecked_ref()
// 为在彼此的不同类型之间转换 JS 值提供支持。
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, Window};

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    let array = Array::new();
    array.push(&"Hello".into());
    array.push(&1.into());

    let mut first_item = None;

    // 用 rust 闭包
    // 这里就测试了一下 js_sys::Array 的一些使用方式
    array.for_each(&mut |obj, idx, _arr| match idx {
        0 => {
            assert_eq!(obj, "Hello");
            first_item = obj.as_string();
        }
        1 => assert_eq!(obj, 1),
        _ => panic!("unknown index: {}", idx),
    });
    assert_eq!(first_item, Some("Hello".to_string()));

    // 这俩函数，执行完了，都没被销毁
    setup_clock(&window, &document)?;
    setup_clicker(&document);

    // wasm 加载完成后，隐藏 loading，并显示操作区域
    document
        .get_element_by_id("loading")
        .expect("should have #loading on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#loading should be an `HtmlElement`")
        .style()
        .set_property("display", "none")?;
    document
        .get_element_by_id("script")
        .expect("should have #script on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#script should be an `HtmlElement`")
        .style()
        .set_property("display", "block")?;

    Ok(())
}

// 定时器，显示时间
// 闭包.as_ref().unchecked_ref()
// 闭包.forget()
fn setup_clock(window: &Window, document: &Document) -> Result<(), JsValue> {
    let current_time = document
        .get_element_by_id("current-time")
        .expect("should have #current-time on the page");
    update_time(&current_time);
    let a = Closure::wrap(Box::new(move || update_time(&current_time)) as Box<dyn Fn()>);
    window
        .set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), 1000)?;
    fn update_time(current_time: &Element) {
        current_time.set_inner_html(&String::from(
            Date::new_0().to_locale_string("en-GB", &JsValue::undefined()),
        ));
    }

    a.forget();

    Ok(())
}

// 给dom绑定点击计数的逻辑
fn setup_clicker(document: &Document) {
    let num_clicks = document
        .get_element_by_id("num-clicks")
        .expect("should have #num-clicks on the page");
    let mut clicks = 0;
    let a = Closure::wrap(Box::new(move || {
        clicks += 1;
        num_clicks.set_inner_html(&clicks.to_string());
    }) as Box<dyn FnMut()>);
    document
        .get_element_by_id("green-square")
        .expect("should have #green-square on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#green-square be an `HtmlElement`")
        .set_onclick(Some(a.as_ref().unchecked_ref()));

    a.forget();
}
