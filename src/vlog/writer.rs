extern crate llhd;

use std::path::Path;
use std::rc::Rc;

use vlog::module::*;

struct VlogWriter {
	modules: Vec<Rc<Module>>,
}

impl VlogWriter {
	fn new () -> VlogWriter {
		VlogWriter {
			modules: vec![],
		}
	}

	fn inst_name(inst: &llhd::value::Value) -> String {
		match inst.name() {
			Some(s) => s.to_string(),
			// TODO: do this with a global counter
			None => "i_23".to_string(),
		}
	}

	fn module_push(&mut self, m : Rc<Module>) -> () {
		// TODO: we should check if the module name is unique
		self.modules.push(m);
	}

	fn module_get(&self, name : String) -> Option<Rc<Module>> {
		// TODO: this should be done more efficient, e.g. hash map as this is
		// currently O(n)
		for m in &self.modules {
			if m.name == name {
				return Some(Rc::clone(&m));
			}
		}

		return None;
	}

	fn process_entity(&mut self, module : &llhd::Module, entity: &llhd::Entity) -> Rc<Module> {
		let mut m = Rc::new(Module::new(entity.name().to_string()));
		self.module_push(Rc::clone(&m));

		for inst in entity.insts() {
			println!("inst {:?}", inst);
			println!("instance name is {:?}", VlogWriter::inst_name(inst));
			match inst.kind() {
				llhd::inst::InstKind::InstanceInst(_, ref target, ref ins, ref outs) => {
					match target {
						&llhd::value::ValueRef::Entity(e) => {
							println!("{:?}", e);
							let en = module.entity(e);
							let inst_mod = match self.module_get(en.name().to_string()) {
								Some(im) => {
									im
								}
								None => {
									panic!("Did not find module {:?}", en.name())
								}
							};

							let mod_inst = Instance::new(
								VlogWriter::inst_name(inst),
								inst_mod);
							Rc::get_mut(&mut m).unwrap().instances.push(mod_inst);
						},
						_ => panic!("Not an entity"),
					}
				},
				_ => println!("Not supported"),
			}
		}

		return m;
	}

	fn from_llhd(&mut self, _filepath : &Path, module : llhd::Module) -> Vec<Rc<Module>> {
		let mut v : Vec<Rc<Module>> = Vec::new();

		for value in module.values() {
			match value {
				&llhd::ValueRef::Entity(e) => {
					let entity = module.entity(e);
					v.push(self.process_entity(&module, entity));
				},
				_ => println!("Something else"),
			}
		}

		return v;
	}
}

pub fn write(filepath : &Path, module : llhd::Module) {
	let mut vlog_writer = VlogWriter::new();
	let m = vlog_writer.from_llhd(filepath, module);
}
