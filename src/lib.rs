use neon::prelude::*;

fn set_object(mut cx: FunctionContext) -> JsResult<JsObject> {
    let meu_nome = cx.argument::<JsString>(0)?;
    let obj = cx.empty_object();
    obj.set(&mut cx, "nome", meu_nome)?;

    Ok(obj)
}

fn handle_object(mut cx: FunctionContext) -> JsResult<JsString> {
    let obj = cx.argument::<JsObject>(0)?;
    let nome: Handle<JsString> = obj.get(&mut cx, "nome")?;

    Ok(nome)
}

fn get_num_cpus(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("set_object", set_object)?;
    cx.export_function("get_num_cpus", get_num_cpus)?;
    cx.export_function("handle_object", handle_object)?;
    Ok(())
}
