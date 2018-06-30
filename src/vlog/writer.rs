extern crate llhd;
use llhd::unit::AsUnitContext;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

use vlog::module::*;

struct VlogWriter {
	modules: Vec<Module>,
	instance_counter : usize,
}

impl VlogWriter {
	fn new () -> VlogWriter {
		VlogWriter {
			modules: vec![],
			instance_counter: 0,
		}
	}

	fn inst_name(&mut self, inst: &llhd::value::Value) -> String {
		match inst.name() {
			Some(s) => s.to_string(),
			None => {
				self.instance_counter += 1;
				let out = format!("i_{}", self.instance_counter);
				println!("No instance name given, default to {}", out);
				out
			},
		}
	}

	fn module_push(&mut self, m : Module) -> ModuleRef {
		// TODO: we should check if the module name is unique
		let id = self.modules.len();

		self.modules.push(m);

		let mut m = self.modules.get_mut(id).unwrap();
		m.id = id;
		return ModuleRef::new(m);
	}

	fn module_get(&self, name: &String) -> Option<ModuleRef> {
		// TODO: this should be done more efficient, e.g. hash map as this is
		// currently O(n)
		for m in &self.modules {
			if &m.name == name {
				return Some(ModuleRef::new(m));
			}
		}

		return None;
	}

	fn module(&self, m_ref : &ModuleRef) -> Option<&Module> {
		self.modules.get(m_ref.id)
	}

	fn module_mut(&mut self, m_ref : &ModuleRef) -> Option<&mut Module> {
		self.modules.get_mut(m_ref.id)
	}

	fn process_type(ty: &llhd::ty::Type) -> Type {
		return match ty.as_ref() {
			&llhd::ty::IntType(ref size) => {
				assert!(*size > 0);
				Rc::new(
					TypeKind::Vector(*size, Rc::new(TypeKind::Scalar)))
			},
			&llhd::ty::SignalType(ref ty) => {
				VlogWriter::process_type(ty)
			},
			&llhd::ty::VectorType(ref size, ref ty) => {
				println!("VectorType found");
				Rc::new(
					TypeKind::Vector(*size,
						VlogWriter::process_type(ty)))
			},
			_ => panic!("Unsupported type found!"),
		};
	}

	fn process_entity_argument(m: &mut Module, arg: &llhd::Value, port_ty: PortType) -> () {
		let name = arg.name().unwrap();
		let ty = VlogWriter::process_type(&arg.ty());
		let w_ref = m.wire_push(Wire::new(name.to_string(), ty));
		m.port_push(Port::new(w_ref, port_ty));
	}

	fn process_entity_arguments(&mut self, entity: &llhd::Entity, m_ref: &ModuleRef) -> () {
		let mut m = self.module_mut(&m_ref).unwrap();
		for arg in entity.inputs() {
			VlogWriter::process_entity_argument(m, arg, PortType::Input);
		}

		for arg in entity.outputs() {
			VlogWriter::process_entity_argument(m, arg, PortType::Output);
		}
	}

	fn process_entity_inst_port_assign(&self, m: &Module, im: &Module, arg: &String, con: &String) -> PortAssign {
		let port = im.port_get(con).unwrap();
		let wire = m.wire_get(arg).unwrap();
		PortAssign::new(port, wire)
	}

	fn process_entity_inst(&mut self, ctx: &llhd::ModuleContext, uctx: &llhd::UnitContext, m_ref: &ModuleRef, inst: &llhd::Inst, e: &llhd::ValueRef, ins: &Vec<llhd::ValueRef>, outs: &Vec<llhd::ValueRef>) -> Instance {
		let target_name = uctx.value(&e).name().unwrap().to_string();
		let inst_mod = match self.module_get(&target_name) {
			Some(im) => {
				im
			}
			None => {
				panic!("Did not find module {:?}", &target_name)
			}
		};

		let eref = match e {
			&llhd::value::ValueRef::Entity(eref) => eref,
			_ => panic!("Should be an entity"),
		};
		let inst_entity = ctx.entity(eref);
		let inst_ectx = llhd::entity::EntityContext::new(ctx, inst_entity);
		let inst_ctx = inst_ectx.as_unit_context();
		let inst_name = self.inst_name(inst);
		let im = self.module(&inst_mod).unwrap();
		let mut instance = Instance::new(inst_name, inst_mod);

		let m = self.module(m_ref).unwrap();

		for (i, arg) in ins.iter().enumerate() {
			let con_input = inst_entity.input(i);
			let con_arg = inst_ctx.argument(con_input);
			let con_value : &llhd::Value = con_arg;
			let con_name = con_value.name().unwrap().to_string();
			let arg_name = uctx.value(arg).name().unwrap().to_string();
			let pa = self.process_entity_inst_port_assign(m, im, &arg_name, &con_name);
			instance.port_assign_push(pa);
		}

		// uah... this is horrible
		for (i, arg) in outs.iter().enumerate() {
			let con_output = inst_entity.output(i);
			let con_arg = inst_ctx.argument(con_output);
			let con_value : &llhd::Value = con_arg;
			let con_name = con_value.name().unwrap().to_string();
			let arg_name = uctx.value(arg).name().unwrap().to_string();
			let pa = self.process_entity_inst_port_assign(m, im, &arg_name, &con_name);
			instance.port_assign_push(pa);
		}

		instance
	}

	fn process_entity_assign(&mut self, ctx: &llhd::UnitContext, m_ref: &ModuleRef, lhs: &llhd::ValueRef, rhs: &llhd::ValueRef) -> Assign {
		let m = self.module_mut(&m_ref).unwrap();
		let lhs_value = ctx.value(lhs).name().unwrap().to_string();
		let lhs_w = m.wire_get(&lhs_value).unwrap();
		let rhs_value = ctx.value(rhs).name().unwrap().to_string();
		let rhs_w = m.wire_get(&rhs_value).unwrap();
		Assign::new(lhs_w, rhs_w)
	}

	fn process_entity(&mut self, ctx: &llhd::ModuleContext, entity: &llhd::Entity) -> () {
		let ectx = llhd::EntityContext::new(ctx, entity);
		let uctx = ectx.as_unit_context();
		let m_ref = self.module_push(Module::new(entity.name().to_string()));

		self.process_entity_arguments(entity, &m_ref);

		for inst in entity.insts() {
			match inst.kind() {
				&llhd::inst::InstKind::DriveInst(ref lhs, ref rhs, _) => {
					let mut a = self.process_entity_assign(uctx, &m_ref, lhs, rhs);
					let mut m = self.module_mut(&m_ref).unwrap();
					m.assign_push(a);
				},
				&llhd::inst::InstKind::InstanceInst(_, ref target, ref ins, ref outs) => {
					let i = self.process_entity_inst(&ctx, uctx, &m_ref, inst, target, ins, outs);
					let mut m = self.module_mut(&m_ref).unwrap();
					m.instance_push(i);
				},
				&llhd::inst::InstKind::SignalInst(ref ty, _) => {
					let value : &llhd::Value = inst;
					let name = value.name().unwrap().to_string();
					let ty = VlogWriter::process_type(ty);
					let wire = Wire::new(name, ty);
					let mut m = self.module_mut(&m_ref).unwrap();
					m.wire_push(wire);
				},
				_ => println!("Not supported"),
			}
		}
	}

	fn from_llhd(&mut self, module: llhd::Module) -> () {
		let ctx = llhd::ModuleContext::new(&module);
		for value in module.values() {
			match value {
				&llhd::ValueRef::Entity(e) => {
					let entity = ctx.entity(e);
					self.process_entity(&ctx, entity);
				},
				_ => println!("Something else"),
			}
		}
	}

	fn write_ports(&self, mut file : &File, m : &Module) {
		let ports = m.ports.iter().map(|ref port| {
			format!("    {}", m.wire(&port.wire).unwrap().name)
		})
		.collect::<Vec<_>>()
		.join(",\n");
		write!(file, "{}\n", ports);
	}

	fn write_wires(&self, mut file : &File, m : &Module) {
		for wire in &m.wires {
			// TODO: vector dimensions are missing
			write!(file, "  wire {};\n", wire.name);
		}
	}

	fn write_instances(&self, mut file : &File, m : &Module) {
		for i in &m.instances {
			let im = self.module(&i.module).unwrap();
			write!(file, "  {} {} (\n", im.name, i.inst_name);

			let ports = i.port_assigns.iter().map(|ref pa| {
				let im = self.module(&i.module).unwrap();
				let port = &im.wire(&im.port(&pa.lhs).unwrap().wire).unwrap().name;
				let con = &m.wire(&pa.rhs).unwrap().name;
				format!("    .{} ( {} )", port, con)
			})
			.collect::<Vec<_>>()
			.join(",\n");

			write!(file, "{}\n", ports);

			write!(file, "  );\n");
		}
	}

	fn write_assigns(&self, mut file : &File, m : &Module) {
		for assign in &m.assigns {
			let lhs = &m.wire(&assign.lhs).unwrap().name;
			let rhs = &m.wire(&assign.rhs).unwrap().name;
			write!(file, "  assign {} = {};\n", lhs, rhs);
		}
	}

	fn write(&self, filepath : &Path) -> () {
		let mut file = match File::create(&filepath) {
			Err(err) => panic!("Could not create {:?}: {}", filepath, err.description()),
			Ok(f) => f,
		};

		file.write(b"// This file was generated by llhd-objcopy\n\n").unwrap();

		for m in &self.modules {
			println!("{:?}", m);
			if m.ports.len() > 0 {
				write!(file, "module {}(\n", m.name);
				self.write_ports(&file, m);
				write!(file, "  );\n");
			} else {
				write!(file, "module {};\n", m.name);
			}

			write!(file, "\n");
			self.write_wires(&file, m);
			write!(file, "\n");
			self.write_instances(&file, m);
			write!(file, "\n");
			self.write_assigns(&file, m);
			write!(file, "endmodule\n\n");
		}
	}
}

pub fn write(filepath : &Path, module : llhd::Module) {
	let mut vlog_writer = VlogWriter::new();
	vlog_writer.from_llhd(module);
	vlog_writer.write(filepath);
}