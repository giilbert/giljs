use rusty_v8 as v8;
use std::env;
use std::fs::read_to_string;
use v8::{json, FunctionTemplate, Local, ObjectTemplate, Value};

fn main() {
    // Initialize V8.
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    {
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
        let handle_scope = &mut v8::HandleScope::new(isolate);
        let context = v8::Context::new(handle_scope);

        let scope = &mut v8::ContextScope::new(handle_scope, context);

        // set the print function
        let global = ObjectTemplate::new(scope);
        global.set(
            v8::String::new(scope, "print").unwrap().into(),
            FunctionTemplate::new(scope, js_print_callback).into(),
        );
        global.set(
            v8::String::new(scope, "input").unwrap().into(),
            FunctionTemplate::new(scope, js_input_callback).into(),
        );

        let context = v8::Context::new_from_template(scope, global);
        let scope = &mut v8::ContextScope::new(scope, context);

        let code_string = read_to_string(env::args().nth(1).unwrap()).unwrap();
        let code = v8::String::new(scope, &code_string).unwrap();

        let script = v8::Script::compile(scope, code, None).unwrap();
        let result = script.run(scope).unwrap();

        // Convert the result to a string and print it.
        let result = result.to_string(scope).unwrap();
        // println!("{}", result.to_rust_string_lossy(scope));
    }

    unsafe {
        v8::V8::dispose();
    }
    v8::V8::shutdown_platform();
}

fn js_print_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    let length = args.length();
    for i in 0..length {
        let current_value = args.get(i);

        let mut out = "".to_string();
        if current_value.is_object() {
            out += &json::stringify(scope, current_value)
                .unwrap()
                .to_rust_string_lossy(scope);
        } else {
            out = current_value.to_rust_string_lossy(scope);
        }

        if i == length - 1 {
            print!("{}", out);
        } else {
            print!("{} ", out);
        }
    }

    print!("\n");
}

fn js_input_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    use std::io::{stdin, stdout, Write};
    print!("{}", args.get(0).to_rust_string_lossy(scope));

    let mut s = String::new();
    let _ = stdout().flush();

    stdin().read_line(&mut s).unwrap();
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }

    return_value.set(v8::String::new(scope, &s).unwrap().into());
}
