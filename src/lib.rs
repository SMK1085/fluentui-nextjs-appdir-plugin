use directive_transform::Config;
use swc_core::{
    ecma::{
        ast::Program,
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata, metadata::TransformPluginMetadataContextKind},
};

#[plugin_transform]
pub fn process_transform(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config_json = &data.get_transform_plugin_config()
        .expect("failed to get plugin data, paths must be provided");
    let config = serde_json::from_str::<Config>(config_json).expect("invalid config for fluentui-next-appdir-directive swc plugin");

    let file_path = match data.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(s) => s,
        None => String::from("")
    };

    // Instead of using the visitor pattern, we'll use the transform crate directly
    // by creating a new module with the necessary imports and functionality
    let mut program = program;
    
    // Check if we need to add the "use client" directive
    let should_add_directive = config.paths.iter().any(|path| file_path.contains(path));
    
    if should_add_directive {
        if let Program::Module(module) = &mut program {
            // Import the necessary types from swc_core
            use swc_core::common::DUMMY_SP;
            use swc_core::ecma::ast::{Expr, ExprStmt, Lit, ModuleItem, Stmt, Str};
            
            // Create the "use client" directive
            let directive = ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: Box::new(Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    value: "use client".into(),
                    raw: None,
                }))),
            }));
            
            // Insert the directive at the beginning of the module
            module.body.insert(0, directive);
        }
    }
    
    program
}
