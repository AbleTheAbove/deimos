use std::error::Error;

use clap::Parser as Clap;

use deimos::middle::typecheck::Typecheck;
use deimos::parser::Parser;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    println!("Deimos compiler v0.0.0.2");
    println!("================================================================");
    println!("This compiler is a stage1 compiler, only used for bootstrapping.");
    println!("================================================================");

    let args = Args::parse();

    let src = std::fs::read_to_string(args.file).expect("Failed to read file");
    drive(&src).unwrap_or_else(|e| {
        eprintln!("{}", e);
    });
}

fn print_errors(errors: Vec<impl Error>) {
    errors.iter().for_each(|e| eprintln!("E: {}", e));
}

fn drive(src: &str) -> anyhow::Result<()> {
    let ast = Parser::parse(src).map_err(|e| {
        print_errors(e);
        anyhow::anyhow!("Parsing failed")
    })?;

    let typed_ast = Typecheck::check(ast).map_err(|e| {
        print_errors(e);
        anyhow::anyhow!("Type checking failed")
    })?;

    //println!("{:#?}", typed_ast);

    // Compile the typed AST to C code
    //let compiler = Transpiler::compile(&typed_ast);
    //println!("{}", compiler);

    Ok(())
}
