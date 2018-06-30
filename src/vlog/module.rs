use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum PortType {
	Input,
	Output,
	// InOut is never used so far, although allowed in Verilog
	// InOut,
}

pub type Type = Rc<TypeKind>;

#[derive(Debug)]
pub enum TypeKind {
	Scalar,
	Vector(usize, Type),
}

#[derive(Debug)]
pub struct Port {
	id: usize,
	pub wire: WireRef,
	pub port_ty: PortType,
}

impl Port {
	pub fn new(w: WireRef, port_ty: PortType) -> Port {
		Port {
			id: 0, // TODO: Nope! Not the right way!
			wire: w,
			port_ty: port_ty,
		}
	}
}

#[derive(Debug)]
pub struct PortRef {
	id: usize,
}

impl PortRef {
	pub fn new(p : &Port) -> PortRef {
		PortRef {
			id: p.id,
		}
	}
}

#[derive(Debug)]
pub struct Wire {
	id: usize,
	pub name: String,
	pub ty: Type,
	pub ports: Vec<PortRef>,
}

impl Wire {
	pub fn new(name: String, ty: Type) -> Wire {
		Wire {
			id: 0, // TODO: Nope! Not the right way!
			name: name,
			ty: ty,
			ports: vec![],
		}
	}

	pub fn port_push(&mut self, p_ref: PortRef) -> () {
		self.ports.push(p_ref);
	}
}

#[derive(Debug, Clone)]
pub struct WireRef {
	id: usize,
}

impl WireRef {
	pub fn new(w : &Wire) -> WireRef {
		WireRef {
			id: w.id,
		}
	}
}

#[derive(Debug)]
pub struct Assign {
	pub id: usize,
	pub lhs: WireRef,
	pub rhs: WireRef,
}

impl Assign {
	pub fn new(lhs: WireRef, rhs: WireRef) -> Assign {
		Assign {
			id: 0, // TODO: Nope! Not the right way!
			lhs: lhs,
			rhs: rhs,
		}
	}
}

#[derive(Debug)]
pub struct PortAssign {
	pub id: usize,
	pub lhs: PortRef,
	pub rhs: WireRef,
}

impl PortAssign {
	pub fn new(lhs: PortRef, rhs: WireRef) -> PortAssign {
		PortAssign {
			id: 0, // TODO: Nope! Not the right way!
			lhs: lhs,
			rhs: rhs,
		}
	}
}

#[derive(Debug)]
pub struct Instance {
	pub id: usize,
	pub inst_name: String,
	pub module: ModuleRef,
	// lhs is within this instance, rhs is within container of instance
	pub port_assigns: Vec<PortAssign>,
}

impl Instance {
	pub fn new(inst_name: String, m: ModuleRef) -> Instance {
		Instance {
			id: 0, // TODO: Nope! Not the right way!
			inst_name: inst_name,
			module: m,
			port_assigns: vec![],
		}
	}

	pub fn port_assign_push(&mut self, a: PortAssign) -> () {
		// TODO: check for duplicates
		let id = self.port_assigns.len();
		self.port_assigns.push(a);
		let amut = self.port_assigns.get_mut(id).unwrap();
		amut.id = id;
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

	pub fn instance_push(&mut self, i: Instance) -> () {
		// TODO: check for duplicates
		let id = self.instances.len();
		self.instances.push(i);
		let imut = self.instances.get_mut(id).unwrap();
		imut.id = id;
	}

	pub fn assign_push(&mut self, a: Assign) -> () {
		// TODO: check for duplicates
		let id = self.assigns.len();
		self.assigns.push(a);
		let amut = self.assigns.get_mut(id).unwrap();
		amut.id = id;
	}

	pub fn wire_push(&mut self, w: Wire) -> WireRef {
		// TODO: check for duplicates
		let id = self.wires.len();
		self.wires.push(w);
		let wmut = self.wires.get_mut(id).unwrap();
		wmut.id = id;
		return WireRef::new(wmut);
	}

	pub fn port_push(&mut self, p: Port) -> PortRef {
		// TODO: check for duplicates
		let id = self.ports.len();
		self.ports.push(p);
		let pmut = self.ports.get_mut(id).unwrap();
		pmut.id = id;
		return PortRef::new(pmut);
	}

	pub fn port_get(&self, name: &String) -> Option<PortRef> {
		for port in &self.ports {
			if &self.wire(&port.wire).unwrap().name == name {
				return Some(PortRef::new(&port));
			}
		}
		return None;
	}

	pub fn wire_get(&self, name: &String) -> Option<WireRef> {
		for wire in &self.wires {
			if &wire.name == name {
				return Some(WireRef::new(&wire));
			}
		}
		return None;
	}

	pub fn port(&self, p_ref : &PortRef) -> Option<&Port> {
		self.ports.get(p_ref.id)
	}

	pub fn wire(&self, w_ref : &WireRef) -> Option<&Wire> {
		self.wires.get(w_ref.id)
	}

	pub fn wire_mut(&mut self, w_ref : &WireRef) -> Option<&mut Wire> {
		self.wires.get_mut(w_ref.id)
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