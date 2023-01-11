use std::io::Cursor;

use bsdiff;
use neon::prelude::*;
use neon::types::buffer::TypedArray;

fn neon_diff(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let old = cx.argument::<JsArrayBuffer>(0)?;
    let new = cx.argument::<JsArrayBuffer>(1)?;

    let mut out = Vec::new();
    bsdiff::diff::diff(old.as_slice(&cx), new.as_slice(&cx), &mut out).expect("failed to diff");

    Ok(JsArrayBuffer::external(&mut cx, out))
}

fn neon_patch(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let old = cx.argument::<JsArrayBuffer>(0)?;
    let patch = cx.argument::<JsArrayBuffer>(1)?;
    let length = cx.argument::<JsNumber>(2)?;

    let mut out = vec![0; length.value(&mut cx) as usize];
    let mut patch_cursor = Cursor::new(patch.as_slice(&cx));
    bsdiff::patch::patch(old.as_slice(&cx), &mut patch_cursor, &mut out).expect("failed to patch");

    Ok(JsArrayBuffer::external(&mut cx, out))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("diff", neon_diff)?;
    cx.export_function("patch", neon_patch)?;
    Ok(())
}
