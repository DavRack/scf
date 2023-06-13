use std::fmt::format;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;
use colored::Colorize;

use regex::Regex;
use clap::Parser as Clap_parser;
use tree_sitter::{Node,Parser};

use self::languages_info::get_lang_for_file;

mod languages_info;

/// Simple Key Remap
#[derive(Clap_parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// code query
    #[arg(name="query", value_name = "regex")]
    query: String,

    /// Select the tree sitter node kind
    #[arg(long, short, name = "kind", value_name = "regex", default_value = ".*")]
    kind: String,

    /// Select the tree sitter node kind
    #[arg(trailing_var_arg = true, name = "files", value_name = "file names")]
    files: Vec<String>,

    /// Select the tree sitter node kind
    #[arg(long, short, value_name = "NUM")]
    max_recursive_depth: Option<u8>,

    /// Select the tree sitter node kind
    #[arg(long, short, default_value = "5", value_name = "NUM")]
    before_context: usize,

    /// Select the tree sitter node kind
    #[arg(long, short, default_value = "5", value_name = "NUM")]
    after_context: usize,
}

fn main() {
    let args = Args::parse();
    let files_to_search = args.files.clone();
    walk_fs(&args, files_to_search, 0);
}

fn walk_fs(args: &Args, files: Vec<String>, depth: u8){

    let reached_max_recursive_depth = match args.max_recursive_depth {
        Some(max_depth) => depth > max_depth,
        None => false,
    };

    if reached_max_recursive_depth {
        return
    }

    for file in files{
        let file_path: PathBuf = PathBuf::from(file);
        if file_path.is_dir(){
            let files_in_dir: Vec<String> = file_path
                .read_dir().expect("cant open dir")
                .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
                .collect()
            ;
            walk_fs(&args, files_in_dir, depth+1)
        }else if file_path.is_file(){
            search_file(file_path, &args);
        }
    }
}

fn search_file(file: PathBuf, args: &Args){
    let lang = get_lang_for_file(file.clone()).unwrap();
    let language = lang.parser;
    let mut parser = Parser::new();
    parser.set_language(language).expect("cant set language for parser");
    let mut source_code = String::new();
    let result = File::open(&file).unwrap().read_to_string(&mut source_code);
    if result.is_err(){
        return;
    }

    let tree = parser.parse(&source_code, None).unwrap();

    let code_query = Regex::new(&args.query).unwrap();
    let kind_query = Regex::new(&args.kind).unwrap();

    let file_name = String::from(file.to_str().unwrap());
    walk_tree(&file_name, &tree.root_node(), source_code.as_bytes(), &kind_query, &code_query, args);
}

fn walk_tree(file_name: &String, node: &Node, source: &[u8], kind_query: &Regex, code_query: &Regex, args: &Args){
    let mut cursor = node.walk();

    let node_childs = node.children(&mut cursor);

    for child in node_childs {
        let node_code = child.utf8_text(source).unwrap();
        let node_kind = child.kind();

        if kind_query.is_match(node_kind) && code_query.is_match(node_code){
            println!("{} => {}", file_name.purple(), node_kind.purple());
            print_code(source, &child, code_query, args);
        }

        walk_tree(file_name, &child, source, kind_query, code_query, args);
    }
}

fn print_code(source_code: &[u8], node: &Node, code_query: &Regex, args: &Args){
    let colorized_node_code = colorize_node(node, source_code, code_query);

    // replace colorized into source code
    let start_byte = node.start_byte();
    let end_byte = node.end_byte();

    let sc = source_code.clone();

    let mut prefix = String::new();
    sc[0..start_byte].to_vec().as_slice().read_to_string(&mut prefix).unwrap();

    let mut sufix = String::new();
    sc[end_byte..].to_vec().as_slice().read_to_string(&mut sufix).unwrap();

    let colored_source_code = format!("{prefix}{colorized_node_code}{sufix}");

    let mut source_code_lines: Vec<String> = colored_source_code.lines().enumerate()
        .map(|line|format!("{: <4} {}",line.0 + 1,line.1.to_string()))
        .collect();

    let line_match = get_match_line(node, code_query, source_code);
    let mut first_line_to_print = 0; 
    let mut last_line_to_print = line_match + args.after_context + 1; 

    if line_match > args.before_context {
        first_line_to_print = line_match - args.before_context;
    }

    if last_line_to_print > source_code_lines.len() {
        last_line_to_print = source_code_lines.len();
    }

    source_code_lines = source_code_lines[first_line_to_print..last_line_to_print].to_vec();
    let print_string = source_code_lines.join("\n");
    // reset the console colors
    let reset = " ".hidden();
    println!("{print_string}{reset}");
}

fn colorize_match(node_code: String, code_query: &Regex) -> String{
    let match_text = code_query.find(&node_code).unwrap();
    let colored_match = format!("{}",String::from(match_text.as_str()).red().bold());
    code_query.replace(&node_code, colored_match).to_string()
}

fn colorize_node(node: &Node, source_code: &[u8], code_query: &Regex) -> String{
    let raw_node_code = node.utf8_text(source_code).unwrap().to_string();
    let node_code = colorize_match(raw_node_code, code_query);
    return node_code.on_bright_black().to_string();
}

fn get_match_line(node: &Node, code_query: &Regex, source_code: &[u8]) -> usize{
    let node_text =node.utf8_text(source_code).unwrap();
    let code_lines: Vec<String> = node_text
        .to_string()
        .lines()
        .map(|line|line.to_string())
        .collect();
    for (i, line) in code_lines.iter().enumerate(){
        if code_query.is_match(line) {
            return i + node.start_position().row;
        }
    }
    return 0
}
