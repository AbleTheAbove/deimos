//! Compiler from typed AST to C code
//!
//! NOTE: This is extremely buggy and incomplete, and only supports a tiny subset of the language
//! which is _just_ enough to get us bootstrapped.

use crate::semant::typed_ast::{TBlock, TExpr, TStmt, TToplevelStmt, TypedAst};
use crate::syntax::ast::{Literal, Numeric, Ty};

use super::pretty_print::PrettyPrinter;

#[derive(Debug, Clone, PartialEq)]
pub struct Transpiler {
    pp: PrettyPrinter,
}

impl Transpiler {
    pub fn new() -> Self {
        Transpiler {
            pp: PrettyPrinter::new(),
        }
    }

    fn compile_ast(&mut self, ast: &TypedAst) {
        // Just copy and paste the standard library into the generated code
        //
        // FIXME: THIS IS EXTREMELY HACKY AND SHOULD BE REPLACED WITH A PROPER MODULE SYSTEM
        // AS SOON AS POSSIBLE, BUT WILL DO FOR A STAGE 1 BOOTSTRAP COMPILER AND EVEN THEN IT'S
        // STILL ATROCIOUS

        let dstring = std::fs::read_to_string("c_lib/dstring.h").expect("Failed to read dstring.h");
        let prelude = std::fs::read_to_string("c_lib/prelude.h").expect("Failed to read prelude.h");

        self.pp.emit_line(&dstring);
        self.pp.emit_line(&prelude);

        for stmt in &ast.nodes.clone() {
            let stmt_str = self.pp.gen_toplevel_stmt(&stmt.target);
            self.pp.emit_line(&stmt_str);
        }
    }

    pub fn compile(ast: &TypedAst) -> String {
        let mut compiler = Transpiler::new();
        compiler.compile_ast(ast);

        let result = compiler.pp.lines.join("\n");
        result
    }
}