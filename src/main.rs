use lalrpop_util::lalrpop_mod;


pub mod formula;


lalrpop_util::lalrpop_mod!(grammar);


fn main() {
  let f = grammar::FormulaParser::new().parse(r"((((~(E. x ~P) /\ T.) \/ F.) -> Q) <-> R)");

  println!("{:?}", f);
}
