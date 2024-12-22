use std::{env, fs, path::PathBuf};

use inkwell::context::Context;
use lexer::lexer::Lexer;
use parser::Parser;
use preprocessor::Preprocessor;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let filename = PathBuf::from(filename);
    let contents = fs::read_to_string(&filename).unwrap();

    let tokens = Lexer::new().run(&contents);
    let ast = Parser::new().run(&tokens);
    let last = Preprocessor::new().run(ast);

    let module_name = filename
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split_once('.')
        .unwrap()
        .0;

    let llvm_cx = Context::create();
    let module_generator = ir::generator::module::LLVMModuleGenerator::new(&llvm_cx, module_name);

    module_generator.generate_from_ast(last);

    module_generator
        .module()
        .print_to_file(filename.parent().unwrap().join(format!("{module_name}.ll")))
        .unwrap();
}
