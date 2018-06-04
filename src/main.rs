extern crate llhd;
extern crate argparse;

use argparse::{ArgumentParser, Store};
use std::io::Read;
use std::fs::File;
use std::path::Path;

mod vlog;

fn main() {
	let mut infile = "".to_string();
	let mut outfile = "".to_string();
	{
		let mut ap = ArgumentParser::new();
		ap.set_description("This is the LLHD objcopy util. This utility converts structural HDL files between different HDL languages.");
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
	let module = parse_input(&inpath).unwrap();
	
	if outfile != "" {
		let outpath = Path::new(&outfile);
		write_output(outpath, module);
	}
}

fn parse_input(filepath : &Path) -> Result<llhd::Module, ()> {
	let mut file = File::open(&filepath).unwrap();
	let mut s = String::new();
	file.read_to_string(&mut s).unwrap();
	let module = llhd::assembly::parse_str(&s).unwrap();
	return Ok(module);
}

fn process_entity(entity: &llhd::Entity) {
	println!("value {:?}", entity.name());
	for inst in entity.insts() {
		println!("inst {:?}", inst);
	}
}

fn write_output(filepath : &Path, module : llhd::Module) {
	let m : vlog::Module;
	for value in module.values() {
		match value {
			&llhd::ValueRef::Entity(e) => {
				let entity = module.entity(e);
				process_entity(entity);
			},
			_ => println!("Something else"),
		}
	}
}
