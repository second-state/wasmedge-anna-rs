use wasmedge_anna;
use wasmedge_quickjs::{Context, JsFn, JsValue};

struct PutFn;
impl JsFn for PutFn {
    // Put: (key: String | Uint8Array, value: String | Uint8Array) -> bool
    fn call(_ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        if argv.len() != 2 {
            return JsValue::Bool(false);
        }
        let key: Vec<u8>;
        let val: Vec<u8>;
        match &argv[0] {
            JsValue::String(s) => key = s.to_string().bytes().collect(),
            JsValue::ArrayBuffer(u8a) => key = u8a.to_vec(),
            _ => return JsValue::Bool(false),
        }
        match &argv[1] {
            JsValue::String(s) => val = s.to_string().bytes().collect(),
            JsValue::ArrayBuffer(u8a) => val = u8a.to_vec(),
            _ => return JsValue::Bool(false),
        }
        return JsValue::Bool(wasmedge_anna::put(key, val));
    }
}

struct GetFn;
impl JsFn for GetFn {
    // Get: (key: String | Uint8Array) -> Uint8Array | null
    fn call(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
        if argv.len() != 1 {
            return JsValue::Null;
        }
        let key: Vec<u8>;
        match &argv[0] {
            JsValue::String(s) => key = s.to_string().bytes().collect(),
            JsValue::ArrayBuffer(u8a) => key = u8a.to_vec(),
            _ => return JsValue::Bool(false),
        }
        let val = wasmedge_anna::get(key);
        val.map_or_else(
            || JsValue::Null,
            |v| JsValue::ArrayBuffer(ctx.new_array_buffer(&v)),
        )
    }
}

fn main() {
    let mut ctx = Context::new();
    let mut obj = ctx.new_object();
    obj.set("put", ctx.new_function::<PutFn>("put").into());
    obj.set("get", ctx.new_function::<GetFn>("get").into());
    ctx.get_global().set("wasmedgeAnna", obj.into());
    let code = r#"
        function ab2str(buf) {
            return String.fromCharCode.apply(null, new Uint8Array(buf));
        }

        function str2ab(str) {
            let buf = new ArrayBuffer(str.length);
            let bufView = new Uint8Array(buf);
            for (let i = 0, strLen = str.length; i < strLen; i++) {
            bufView[i] = str.charCodeAt(i);
            }
            return buf;
        }

        let value = "bar " + Math.random();
        print("put result:", wasmedgeAnna.put("foo", str2ab(value)));
        let foo_val = wasmedgeAnna.get("foo");
        print("foo:", ab2str(foo_val)); // should be "bar " + a random number
        let bar_val = wasmedgeAnna.get("bar");
        print("bar:", bar_val); // should be null
    "#;
    ctx.eval_global_str(code);
}
