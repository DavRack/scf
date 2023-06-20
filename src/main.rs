use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use colored::Colorize;
use std::path::Path;
use regex::Regex;
use clap::Parser as Clap_parser;
use tree_sitter::{Node,Parser};
use std::fs::{create_dir};


use self::languages_info::get_lang_for_file;

mod languages_info;

/// Simple Key Remap
#[derive(Clap_parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// code query
    #[arg(name="query", value_name = "regex")]
    query: String,

    /// Select the tree sitter node kind
    #[arg(long, short, name = "kind", value_name = "regex")]
    kind: Option<String>,

    /// Select the files to search
    #[arg(trailing_var_arg = true, name = "files", value_name = "file names")]
    files: Option<Vec<String>>,

    /// Maximun recursive folder find depth
    #[arg(long, short, value_name = "Int")]
    max_recursive_depth: Option<u8>,

    /// Config file path
    #[arg(long, short, value_name = "String")]
    config_file_path: Option<String>,

    /// Before match context lines
    #[arg(long, short, default_value = "5", value_name = "Int")]
    before_context: usize,

    /// After match context lines
    #[arg(long, short, default_value = "5", value_name = "Int")]
    after_context: usize,

    /// Show all matches (if false will show the top most node match only)
    #[arg(long, short, default_value = "false", value_name = "Int")]
    show_all_matches: bool,
}

fn main() {
    let args = Args::parse();
    // println!("{:?}", args);
    let path = get_config_file_path(&args);
    let config_file = match path {
        Some(value) => parse_config_file(value),
        None => serde_json::Value::Null,
    };
    let files_to_search = match args.files.clone(){
        Some(files) => files,
        None => vec![".".to_string()],
    };
    walk_fs(&args, &config_file, files_to_search, 0);
}

fn get_config_file_path(args: &Args) -> Option<String>{
    return match &args.config_file_path {
        Some(value) => Some(value.to_string()),
        None => {
            let home_dir_path = std::env::var("HOME").expect("can't get home path dir, the env variable 'HOME' is not set");
            let scf_config_file_path = Path::new(&home_dir_path).join(".config/scf/config.hcl");
            match Path::is_file(&scf_config_file_path) {
                true => Some(scf_config_file_path.to_str().unwrap().to_string()),
                false => None
            }

        }

    };
}

fn parse_config_file(path: String) -> serde_json::Value{
    let file_path: PathBuf = PathBuf::from(path);
    let mut file_content_buffer = String::new();
    let file_content = match File::open(file_path){
        Ok(mut file) => {
            file.read_to_string(&mut file_content_buffer).unwrap();
            file_content_buffer
        },
        Err(e) => {
            println!("Cannot open config file: {:?}", e.to_string());
            String::from("")
        },
    };
    let config: serde_json::Value = hcl::de::from_str(&file_content).unwrap();
    return config;
}

fn walk_fs(args: &Args, config_file: &serde_json::Value, files: Vec<String>, depth: u8){

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
            walk_fs(args, config_file,files_in_dir, depth+1);
        }else if file_path.is_file(){
            search_file(file_path, &args.clone(), config_file);
        }
    }
}

fn search_file(file: PathBuf, args: &Args, config_file: &serde_json::Value){
    let lang = match get_lang_for_file(file.clone()){
        Ok(value) => value,
        Err(_) => return,
    };
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
    let kind_regex = match &args.kind {
        Some(value) => {
            let config_lang_alias = &config_file["alias"][lang.name][value];
            let config_global_alias = &config_file["alias"]["global"][value];
            match config_lang_alias {
                serde_json::Value::String(string) => string.to_string(),
                _ => match config_global_alias{
                    serde_json::Value::String(string) => string.to_string(),
                    _ => value.to_string()
                }
            }
        },
        None => ".*".to_string(),
    };
    let kind_query = Regex::new(&kind_regex).unwrap();

    let file_name = String::from(file.to_str().unwrap());
    walk_tree(&file_name, &tree.root_node(), source_code.as_bytes(), &kind_query, &code_query, args, vec![]);
}

fn walk_tree(file_name: &String, node: &Node, source: &[u8], kind_query: &Regex, code_query: &Regex, args: &Args, node_history: Vec<Node>){
    let mut cursor = node.walk();

    let node_childs = node.children(&mut cursor);

    for child in node_childs {
        let node_code = child.utf8_text(source).unwrap();

        let mut n = node_history.clone();
        n.push(child.clone());
        let node_kind = get_node_kind(&n);
        // println!("{:?}", node_kind);

        if kind_query.is_match(&node_kind) && code_query.is_match(node_code){
            println!("{} => {}", file_name.purple(), node_kind.purple());
            print_code(source, &child, code_query, args);
            // println!("{:?}", n);
            if !args.show_all_matches {
                return
            }
        }

        walk_tree(file_name, &child, source, kind_query, code_query, args, n);
    }
}

fn get_node_kind(node_history: &Vec<Node>) -> String{
    let nodes_kind: Vec<&str> = node_history
        .iter()
        .map(|node| node.kind())
        .collect();
    return nodes_kind.join("/")
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
