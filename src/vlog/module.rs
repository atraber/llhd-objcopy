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
	name: String,
	module: Rc<Module>,
}

pub struct Module {
	ports: Vec<Port>,
	wires: Vec<Rc<Wire>>,
	assigns: Vec<Assign>,
	instances: Vec<Instance>,
}
