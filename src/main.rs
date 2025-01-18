use lalrpop_util::lalrpop_mod;
use classical_logic_rs::formula;


lalrpop_mod!(grammar);


#[allow(non_snake_case)]
fn main() {
  let F: formula::Formula = grammar::FormulaParser::new().parse(r"((P /\ Q) \/ R)").unwrap();

  let V = std::collections::HashMap::from([
    ("P".to_string(), true),
    ("Q".to_string(), true),
    ("R".to_string(), false),
  ]);

  println!("{}", F);
  println!("{:?}", F.atom_vec());
  println!("{:?}", formula::eval(&F, &V));
}
