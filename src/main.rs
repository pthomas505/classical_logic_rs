use lalrpop_util::lalrpop_mod;
use classical_logic_rs::formula;


lalrpop_mod!(grammar);


#[allow(non_snake_case)]
fn main() {
  let F: formula::Formula = grammar::FormulaParser::new().parse(r"((((~(E. x ~P) /\ T.) \/ F.) -> Q) <-> R)").unwrap();

  println!("{}", F);
  println!("{:?}", F.atom_vec());
}
