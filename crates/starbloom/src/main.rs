use std::{env, fs, path::PathBuf, rc::Rc};

use inkwell::context::Context;
use lexer::lexer::lexer;
use parser::parser::parse;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let filename = PathBuf::from(filename);
    let contents = fs::read_to_string(&filename).unwrap();

    let tokens = lexer(&contents);
    let (ast, _) = parse(&tokens);

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

    module_generator.generate_from_ast(Rc::new(ast));

    module_generator
        .module()
        .print_to_file(filename.parent().unwrap().join(format!("{module_name}.ll")))
        .unwrap();
}
