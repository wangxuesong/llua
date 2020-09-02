mod chunk;
mod vm;

use clap::{Arg, App};
use crate::chunk::binary::{Chunk, Header, Prototype, Constant};
use crate::vm::{Instruction};

fn main() {
    let matches = App::new("llua")
        .version("0.1")
        .about("for sweet hui")
        .arg(Arg::with_name("INPUT")
            .about("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("list")
            .short('l')
            .long("list")
            // .multiple(true)
            .about("list binary file content"))
        .get_matches();

    if matches.is_present("list") {
        let input = matches.value_of("INPUT").unwrap();
        show_binary(input);
    }
}

fn show_binary(input: &str) {
    let content = std::fs::read(input).unwrap();
    let parse_result = Chunk::parse(content.as_slice()).unwrap();
    let chunk: Chunk = parse_result.1;
    let head = chunk.header;
    print_header(&head);
    let proto = chunk.main;
    print_proto(&proto);
}

fn print_header(head: &Header) {
    println!("lua version: {:x}", head.version);
}

fn print_proto(f: &Prototype) {
    print_proto_header(f);
    print_code(f);
    print_detail(f);
    for prototype in &f.prototypes {
        print_proto(&prototype);
    }
}

fn print_proto_header(f: &Prototype) {
    let func_type = if f.line_defined > 0 { "function" } else { "main" };
    let vararg_flag = if f.is_vararg > 0 { "+" } else { "" };
    let source = f.source.as_ref().map(|x| x.as_str()).unwrap_or("");
    print!("\n{}", func_type);
    print!(" <{}:{},{}>", source, f.line_defined, f.last_line_defined);
    print!(" ({} instructions)\n", f.code.len());
    print!("{}{} params", f.num_params, vararg_flag);
    print!(", {} slots", f.max_stack_size);
    print!(", {} upvalues", f.upvalues.len());
    print!(", {} locals", f.loc_vars.len());
    print!(", {} constants", f.constants.len());
    print!(", {} functions\n", f.prototypes.len());
}

fn print_code(f: &Prototype) {
    for pc in 0..f.code.len() {
        let line = f.line_info.get(pc).map(|n| n.to_string()).unwrap_or(String::new());
        println!("\t{}\t[{}]\t{}", pc + 1, line, f.code[pc].opname());
    }
}

fn print_detail(f: &Prototype) {
    print_consts(f);
    print_locals(f);
    print_upvals(f)
}

fn print_consts(f: &Prototype) {
    let n = f.constants.len();
    println!("constants ({}):", n);
    for i in 0..n {
        print_const(i + 1, &f.constants[i]);
    }
}

fn print_const(n: usize, k: &Constant) {
    use crate::chunk::binary::ConstantValue::*;
    match &k.const_value {
        Nil => println!("\t{}\tnil", n),
        Boolean(b) => println!("\t{}\t{}", n, b),
        Number(x) => println!("\t{}\t{}", n, x),
        Integer(i) => println!("\t{}\t{}", n, i),
        ShortStr(s) => println!("\t{}\t{:?}", n, s.value),
    }
}

fn print_locals(f: &Prototype) {
    let n = f.loc_vars.len();
    println!("locals ({}):", n);
    for i in 0..n {
        let var = &f.loc_vars[i];
        println!("\t{}\t{}\t{}\t{}", i, var.var_name.value, var.start_pc + 1, var.end_pc + 1);
    }
}

fn print_upvals(f: &Prototype) {
    let n = f.upvalues.len();
    println!("upvalues ({}):", n);
    for i in 0..n {
        let upval = &f.upvalues[i];
        let name = f.upvalue_names.get(i).map(|x| x.value.as_str()).unwrap_or("");
        println!("\t{}\t{}\t{}\t{}", i, name, upval.instack, upval.idx);
    }
}