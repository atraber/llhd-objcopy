extern crate llhd;
extern crate argparse;

use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod vlog;

fn main() {
	let mut infile = "".to_string();
	let mut outfile = "".to_string();
	{
		let mut ap = ArgumentParser::new();
		ap.set_description(r#"
			This is the LLHD objcopy util.
			This utility converts structural HDL files between different HDL languages.
			"#);
		ap.refer(&mut infile)
			.add_argument("in-file", Store, "input file").required();
		ap.refer(&mut outfile)
			.add_argument("out-file", Store, "output file");
		ap.parse_args_or_exit();
	}

	if !infile.ends_with(".llhd") {
		panic!("File does not end in .llhd. Currently only llhd input files are supported");
	}

	let inpath = Path::new(&infile);
	let module = match parse_input(&inpath) {
		Ok(m) => m,
		Err(s) => panic!("Failed to parse input: {}", s),
	};

	println!("Successfully parsed input");

	if outfile != "" {
		let outpath = Path::new(&outfile);
		vlog::write(outpath, module);
	} else {
		println!("No output file specified; Nothing to do");
	}
}

fn parse_input(filepath : &Path) -> Result<llhd::Module, String> {
	let mut file = File::open(&filepath).unwrap();
	let mut s = String::new();
	file.read_to_string(&mut s).unwrap();
	return llhd::assembly::parse_str(&s);
}