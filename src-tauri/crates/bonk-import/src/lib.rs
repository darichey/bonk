use deno_ast::{MediaType, ParseParams, SourceTextInfo};
use deno_core::{
    anyhow::{bail, Context, Error},
    error::AnyError,
    futures::FutureExt,
    v8, JsRuntime, ModuleLoader, ModuleSource, ModuleSourceCode, ModuleSourceFuture,
    ModuleSpecifier, ModuleType, PollEventLoopOptions, ResolutionKind, RuntimeOptions,
};
use std::fs;
use std::pin::Pin;
use std::rc::Rc;

struct TsModuleLoader;

impl ModuleLoader for TsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, Error> {
        Ok(deno_core::resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
    ) -> Pin<Box<ModuleSourceFuture>> {
        let module_specifier = module_specifier.clone();
        async move {
            let path = module_specifier.to_file_path().unwrap();

            let media_type = MediaType::from_path(&path);

            if media_type != MediaType::TypeScript {
                bail!("Unsupported file type: {}", media_type);
            }

            let code = {
                let code = fs::read_to_string(&path)?;
                let parsed = deno_ast::parse_module(ParseParams {
                    specifier: module_specifier.to_string(),
                    text_info: SourceTextInfo::from_string(code),
                    media_type,
                    capture_tokens: false,
                    scope_analysis: false,
                    maybe_syntax: None,
                })?;
                parsed.transpile(&Default::default())?.text
            };

            Ok(ModuleSource::new(
                ModuleType::JavaScript,
                ModuleSourceCode::Bytes(code.into_bytes()),
                &module_specifier,
            ))
        }
        .boxed_local()
    }
}

struct Importer {
    js_runtime: JsRuntime,
    mod_id: usize,
}

impl Importer {
    async fn new(importer_path: &str) -> Result<Self, AnyError> {
        let module = deno_core::resolve_path(
            importer_path,
            &std::env::current_dir().context("Unable to get CWD")?,
        )?;
        let mut js_runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(TsModuleLoader)),
            ..Default::default()
        });
        let mod_id = js_runtime.load_side_module(&module, None).await?;
        let result = js_runtime.mod_evaluate(mod_id);

        js_runtime
            .run_event_loop(PollEventLoopOptions::default())
            .await?;

        result.await?;

        Ok(Self { js_runtime, mod_id })
    }

    fn add(&mut self, x: i32, y: i32) -> Result<i32, AnyError> {
        let global = self.js_runtime.get_module_namespace(self.mod_id)?;
        let scope = &mut self.js_runtime.handle_scope();
        let local = v8::Local::new(scope, global);

        let key = v8::String::new(scope, "add").unwrap().into();
        let func = local.get(scope, key).unwrap();
        let func = v8::Local::<v8::Function>::try_from(func).unwrap();

        let x = v8::Integer::new(scope, x).into();
        let y = v8::Integer::new(scope, y).into();
        let func_res = func.call(scope, local.into(), &[x, y]).unwrap();

        Ok(func_res.int32_value(scope).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::Importer;

    #[test]
    fn test() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            let mut importer = Importer::new("./example.ts").await.unwrap();
            assert_eq!(importer.add(5, 2).unwrap(), 7);
        });
    }
}
