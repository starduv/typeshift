mod open_api_v3;
mod typescript;

use std::{fs::File, io::Write, path::PathBuf};

use neon::{prelude::*, result::Throw};
use serde_json::json;

use self::{open_api_v3::OpenApiV3, typescript::TsNode};

fn merge_schemas(
    open_api: &OpenApiV3,
    open_api_handle: Handle<JsObject>,
    cx: &mut FunctionContext,
) -> Result<String, Throw> {
    let base_handle: Handle<JsString> = open_api_handle.get(cx, "base")?;
    let base = serde_json::from_str(&base_handle.value(cx)).expect("Could not deserialize base schema");
    let mut generated = json!(open_api);
    merge(&mut generated, &base);
    Ok(generated.to_string())
}

fn merge(target: &mut serde_json::Value, overlay: &serde_json::Value) {
    if target.is_object() && overlay.is_object() {
        let target = target.as_object_mut().unwrap();
        let overlay = overlay.as_object().unwrap();
        for (key, value) in overlay {
            if target.contains_key(key) {
                let gen_value = target.get_mut(key).unwrap();
                merge(gen_value, value);
            } else {
                target.insert(key.clone(), value.clone());
            }
        }
    } else {
        target.clone_from(overlay);
    }
}

fn add_api_paths<'cx>(open_api: &mut OpenApiV3, node: &mut TsNode<'cx>, cx: &mut FunctionContext) -> Result<(), Throw> {
    let api_paths = node.get_api_paths(cx)?;

    Ok(())
}

fn generate_schema(open_api_handle: Handle<JsObject>, cx: &mut FunctionContext) -> Result<String, Throw> {
    let get_ast = open_api_handle.get::<JsFunction, FunctionContext, &str>(cx, "get_ast")?;
    let paths = open_api_handle.get::<JsArray, FunctionContext, &str>(cx, "paths")?;
    let mut open_api = OpenApiV3::new();

    for path in paths.to_vec(cx)? {
        let path = path.downcast_or_throw::<JsString, FunctionContext>(cx)?;
        let ast = get_ast.call_with(cx).arg(path).apply::<JsObject, FunctionContext>(cx)?;
        add_api_paths(&mut open_api, &mut TsNode::new(ast), cx)?;
    }

    merge_schemas(&open_api, open_api_handle, cx)
}

pub fn generate_openapi(
    schemas_result: Handle<JsObject>,
    options_handle: Handle<JsObject>,
    cx: &mut FunctionContext,
) -> Result<(), Throw> {
    let schema_result: Handle<JsObject> = cx.empty_object();
    if let Some(open_api_handle) = options_handle.get_opt(cx, "openApi")? as Option<Handle<JsObject>> {
        let schema: String = generate_schema(open_api_handle, cx)?;

        if let Some(output_handle) = open_api_handle.get_opt::<JsString, FunctionContext, &str>(cx, "output")? {
            let filepath = match options_handle.get_opt::<JsString, FunctionContext, &str>(cx, "cwd")? {
                Some(cwd) => {
                    let mut path = PathBuf::from(cwd.value(cx));
                    path.push(output_handle.value(cx));
                    path
                }
                None => PathBuf::from(output_handle.value(cx)),
            };

            let mut file: File = File::create(filepath.clone()).expect("Could not create filepath: ");
            file.write_all(schema.as_bytes()).expect("Could not write to file");

            let is_file = cx.boolean(true);
            let filepath = cx.string(filepath.to_str().unwrap());
            schema_result.set(cx, "isFile", is_file)?;
            schema_result.set(cx, "filepath", filepath)?;
        } else {
            let is_file = cx.boolean(false);
            let schema = cx.string(schema);
            schema_result.set(cx, "isFile", is_file)?;
            schema_result.set(cx, "schema", schema)?;
        }
    }

    schemas_result.set(cx, "openApi", schema_result)?;

    Ok(())
}