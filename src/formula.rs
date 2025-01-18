

#[derive(Debug, PartialEq, Eq)]
pub enum Formula {
  False,
  True,
  Atom(String),
  Not(Box<Formula>),
  And(Box<Formula>, Box<Formula>),
  Or(Box<Formula>, Box<Formula>),
  Imp(Box<Formula>, Box<Formula>),
  Iff(Box<Formula>, Box<Formula>),
  Forall(String, Box<Formula>),
  Exists(String, Box<Formula>),
}

#[allow(non_snake_case)]
impl std::fmt::Display for Formula {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Formula::False => write!(f, "F."),
      Formula::True => write!(f, "T."),
      Formula::Atom(X) => write!(f, "{}", X),
      Formula::Not(phi) => write!(f, "~ {}", *phi),
      Formula::And(phi, psi) => write!(f, "({} /\\ {})", *phi, *psi),
      Formula::Or(phi, psi) => write!(f, "({} \\/ {})", *phi, *psi),
      Formula::Imp(phi, psi) => write!(f, "({} -> {})", *phi, *psi),
      Formula::Iff(phi, psi) => write!(f, "({} <-> {})", *phi, *psi),
      Formula::Forall(x, phi) => write!(f, "(A. {} {})", x, *phi),
      Formula::Exists(x, phi) => write!(f, "(E. {} {})", x, *phi),
    }
  }
}


impl Formula {
  #[allow(non_snake_case)]
  pub fn atom_vec(&self) -> Vec<String> {
    match self {
      Formula::False => vec![],
      Formula::True => vec![],
      Formula::Atom(X) => vec![X.to_string()],
      Formula::Not(phi) => phi.atom_vec(),
      Formula::And(phi, psi) => {
        let mut xs = phi.atom_vec();
        xs.append(&mut psi.atom_vec());
        xs
      }
      Formula::Or(phi, psi) => {
        let mut xs = phi.atom_vec();
        xs.append(&mut psi.atom_vec());
        xs
      }
      Formula::Imp(phi, psi) => {
        let mut xs = phi.atom_vec();
        xs.append(&mut psi.atom_vec());
        xs
      }
      Formula::Iff(phi, psi) => {
        let mut xs = phi.atom_vec();
        xs.append(&mut psi.atom_vec());
        xs
      }
      Formula::Forall(_, phi) => phi.atom_vec(),
      Formula::Exists(_, phi) => phi.atom_vec(),
    }
  }
}


#[allow(non_snake_case)]
pub fn eval(F: &Formula, V: &dyn Fn(&String) -> bool) -> bool {
  match F {
    Formula::False => false,
    Formula::True => true,
    Formula::Atom(X) => V(X),
    Formula::Not(phi) => ! eval(phi, V),
    Formula::And(phi, psi) => eval(phi, V) && eval(psi, V),
    Formula::Or(phi, psi) => eval(phi, V) || eval(psi, V),
    Formula::Imp(phi, psi) => (! eval(phi, V)) || eval(psi, V),
    Formula::Iff(phi, psi) => eval(phi, V) == eval(psi, V),
    Formula::Forall(_, phi) => eval(phi, V),
    Formula::Exists(_, phi) => eval(phi, V)
  }
}
