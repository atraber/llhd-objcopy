use std::rc::Rc;

pub enum PortType {
	Input,
	Output,
	InOut,
}

pub struct Port {
	wire: Rc<Wire>,
	ty: PortType,
}

pub struct Wire {
	name: String,
}

pub struct Assign {
	lhs: Rc<Wire>,
	rhs: Rc<Wire>,
}

pub struct Instance {
	inst_name: String,
	module: Rc<Module>,
}

impl Instance {
	pub fn new(inst_name : String, m : Rc<Module>) -> Instance {
		Instance {
			inst_name: inst_name,
			module: m,
		}
	}
}

pub struct Module {
	pub name: String,
	pub ports: Vec<Port>,
	pub wires: Vec<Rc<Wire>>,
	pub assigns: Vec<Assign>,
	pub instances: Vec<Instance>,
}

impl Module {
	pub fn new(name: String) -> Module {
		Module {
			name: name,
			ports: vec![],
			wires: vec![],
			assigns: vec![],
			instances: vec![],
		}
	}
}