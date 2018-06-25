#[derive(Debug)]
pub enum PortType {
	Input,
	Output,
	InOut,
}

#[derive(Debug)]
pub struct Port {
	id: usize,
	wire: WireRef,
	ty: PortType,
}

#[derive(Debug)]
pub struct PortRef {
	id: usize,
}

#[derive(Debug)]
pub struct Wire {
	id: usize,
	name: String,
}

#[derive(Debug)]
pub struct WireRef {
	id: usize,
}

#[derive(Debug)]
pub struct Assign {
	id: usize,
	lhs: WireRef,
	rhs: WireRef,
}

#[derive(Debug)]
pub struct AssignRef {
	id: usize,
}

#[derive(Debug)]
pub struct Instance {
	pub id: usize,
	pub inst_name: String,
	pub module: ModuleRef,
}

impl Instance {
	pub fn new(inst_name : String, m : ModuleRef) -> Instance {
		Instance {
			id: 0, // TODO: Nope! Not the right way!
			inst_name: inst_name,
			module: m,
		}
	}
}

#[derive(Debug)]
pub struct InstanceRef {
	pub id: usize,
}

impl InstanceRef {
	pub fn new(i : &Instance) -> InstanceRef {
		InstanceRef {
			id: i.id,
		}
	}
}

#[derive(Debug)]
pub struct Module {
	pub id: usize,
	pub name: String,
	pub ports: Vec<Port>,
	pub wires: Vec<Wire>,
	pub assigns: Vec<Assign>,
	pub instances: Vec<Instance>,
}

impl Module {
	pub fn new(name : String) -> Module {
		Module {
			id: 0, // TODO: Nope! Not the right way!
			name: name,
			ports: vec![],
			wires: vec![],
			instances: vec![],
			assigns: vec![],
		}
	}
}

#[derive(Debug)]
pub struct ModuleRef {
	pub id: usize,
}

impl ModuleRef {
	pub fn new(m : &Module) -> ModuleRef {
		ModuleRef {
			id: m.id,
		}
	}
}