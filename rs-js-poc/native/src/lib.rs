use neon::prelude::*;
use serde_json;
use rslib;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn pass_through_string(mut cx: FunctionContext) -> JsResult<JsString> {
    let input_string = match cx.argument_opt(0) {
        Some(arg) => arg.downcast::<JsString>().or_throw(&mut cx)?.value(),
        // Default to empty String if no value is given
        None => String::new(),
    };

    let msg = format!("input string: {}", input_string);
    Ok(cx.string(msg))
}

fn get_graph(mut cx: FunctionContext) -> JsResult<JsString> {
    let g = rslib::get_graph();
    let serialized_g = serde_json::to_string_pretty(&g).unwrap();
    Ok(cx.string(serialized_g))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_function("pass_through_string", pass_through_string)?;
    cx.export_function("get_graph", get_graph)?;
    Ok(())
});

// register_module!(mut m, {
//     m.export_function("printFunction", print_function)?;
//     m.export_function("add1ToArgument", add_1_to_argument)?;
//     m.export_function("getArgsLen", get_args_len)?;
//     m.export_function("argsOpt", args_opt)?;
//     m.export_function("defaultArgs", default_args)?;
//     m.export_function("acceptsJsArrays", accepts_js_arrays)?;
//     m.export_function("acceptsJsObjects", accepts_js_objects)?;
//     Ok(())
// });
