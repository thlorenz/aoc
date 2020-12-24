use std::collections::{HashMap, HashSet};

use regex::Regex;
use Operation::*;
use Stmt::*;
use Value::*;

fn main() {
    let program = include_str!("./input.txt");
    let circuit = Circuit::from_program(true, &program);
    let slots = circuit.run();

    let a_part1 = slots.get("a").unwrap();
    println!("part 1: {:#?}", a_part1);

    // part 2
    let mut slots = HashMap::new();
    slots.insert("b".to_string(), *a_part1);
    let slots = circuit.run_with_slots(slots);

    println!("part 2: {:#?}", slots.get("a").unwrap());
}

type SlotValue = u16; // 16 bit signal
type Slots = HashMap<String, SlotValue>;

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Literal(SlotValue),
    Slot(String),
}

impl From<&str> for Value {
    fn from(str: &str) -> Self {
        match str.parse::<SlotValue>() {
            Ok(n) => Literal(n),
            _ => Slot(str.to_string()),
        }
    }
}

impl Value {
    fn reference(&self) -> Option<String> {
        match self {
            Slot(s) => Some(s.clone()),
            Literal(_) => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    And,
    Or,
    LShift,
    RShift,
    Not,
}

impl From<&str> for Operation {
    fn from(str: &str) -> Self {
        match str {
            "AND" => And,
            "OR" => Or,
            "LSHIFT" => LShift,
            "RSHIFT" => RShift,
            "NOT" => Not,
            _ => panic!("Unkown operation '{}'", str),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Dependencies(Vec<String>);

impl Default for Dependencies {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Dependencies {
    fn from_values(values: Vec<&Value>) -> Self {
        let deps: Vec<String> = values
            .into_iter()
            .filter_map(|val| val.reference())
            .collect();
        Dependencies(deps)
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Stmt {
    // assignee, value
    Assign(String, Value, Dependencies),
    // assignee = value1 operation value2
    AssignExpr(String, Value, Operation, Value, Dependencies),
    // assignee = !value
    SetNot(String, Value, Dependencies),
}

struct Circuit {
    statements: Vec<Stmt>,
    stmt_assign: Regex,
    stmt_expr_assign: Regex,
    stmt_not: Regex,
    halt_and_catch_fire: bool,
}

impl Circuit {
    fn new(halt_and_catch_fire: bool) -> Self {
        let stmt_assign: Regex = Regex::new(r"^([^\W]+) -> ([^\W]+)").unwrap();
        let stmt_expr_assign: Regex =
            Regex::new(r"^([^\W]+) (AND|OR|LSHIFT|RSHIFT) ([^\W]+) -> ([^\W]+)").unwrap();
        let stmt_not: Regex = Regex::new(r"^NOT ([^\W]+) -> ([^\W]+)").unwrap();
        Self {
            statements: vec![],
            stmt_assign,
            stmt_expr_assign,
            stmt_not,
            halt_and_catch_fire,
        }
    }

    fn from_program(halt_and_catch_fire: bool, program: &str) -> Self {
        let mut circuit = Circuit::new(halt_and_catch_fire);
        circuit.add_statements(&program);
        circuit.sort_statements();
        circuit
    }

    /*
    * In order for values to be resolved properly, we need to make sure we resolved its
    * depencencies first. This reordering is expensive, but allows us to rerun the program with sorted
    * statements multiple times.
    * Additionally once we do that all values are resolved.

    * An alternative would be to focus on a value we need to resolve and walk its dependencies
    * backwards. However we'd have to do this for each value
    */
    fn sort_statements(&mut self) {
        let mut stmts = self.statements.clone();
        let mut sorted = Vec::<Stmt>::with_capacity(stmts.len());
        let mut resolved_symbols = HashSet::<String>::new();

        while stmts.len() > 0 {
            let remaining: Vec<Stmt> = stmts
                .into_iter()
                .filter_map(|stmt| {
                    let (assignee, deps) = match &stmt {
                        Assign(assignee, _, deps) => (assignee, &deps.0),
                        AssignExpr(assignee, _, _, _, deps) => (assignee, &deps.0),
                        SetNot(assignee, _, deps) => (assignee, &deps.0),
                    };

                    let all_deps_resolved = deps.iter().all(|s| resolved_symbols.get(s).is_some());

                    if all_deps_resolved {
                        resolved_symbols.insert(assignee.clone());
                        sorted.push(stmt);
                        None
                    } else {
                        Some(stmt)
                    }
                })
                .collect();
            stmts = remaining;
        }

        self.statements = sorted;
    }

    fn run_with_slots(&self, mut slots: Slots) -> Slots {
        for stmt in &self.statements {
            Circuit::process_stmt(&mut slots, &stmt)
        }
        slots
    }

    fn run(&self) -> Slots {
        self.run_with_slots(HashMap::new())
    }

    fn add_statements(&mut self, program: &str) {
        let lines = program.lines();
        for line in lines {
            self.add_statement(line);
        }
    }

    fn add_statement(&mut self, line: &str) {
        let line = line.trim();
        if let Some(stmt) = self.stmt_assign(line) {
            self.statements.push(stmt);
        } else if let Some(stmt) = self.stmt_assign_expr(line) {
            self.statements.push(stmt);
        } else if let Some(stmt) = self.stmt_not(line) {
            self.statements.push(stmt);
        } else if self.halt_and_catch_fire {
            panic!("Unable to add '{}'", line);
        }
    }

    fn stmt_assign(&self, line: &str) -> Option<Stmt> {
        // 123 -> x
        // x -> y
        if self.stmt_assign.is_match(line) {
            let captures = self.stmt_assign.captures(line).expect("regex lib broken");
            let (val, assignee) = (
                captures.get(1).expect("capture 1").as_str(),
                captures.get(2).expect("capture 2").as_str(),
            );
            let val: Value = val.into();
            let dependencies = Dependencies::from_values(vec![&val]);
            let stmt = Assign(assignee.to_string(), val, dependencies);
            Some(stmt)
        } else {
            None
        }
    }

    fn stmt_assign_expr(&self, line: &str) -> Option<Stmt> {
        // x AND y -> d
        if self.stmt_expr_assign.is_match(line) {
            let captures = self
                .stmt_expr_assign
                .captures(line)
                .expect("regex lib broken");
            let (op_left, op, op_right, assignee) = (
                captures.get(1).expect("capture 1").as_str(),
                captures.get(2).expect("capture 2").as_str(),
                captures.get(3).expect("capture 3").as_str(),
                captures.get(4).expect("capture 4").as_str(),
            );
            let op_left: Value = op_left.into();
            let op_right: Value = op_right.into();
            let dependencies = Dependencies::from_values(vec![&op_left, &op_right]);
            let stmt = AssignExpr(
                assignee.to_string(),
                op_left,
                op.into(),
                op_right,
                dependencies,
            );
            Some(stmt)
        } else {
            None
        }
    }

    fn stmt_not(&self, line: &str) -> Option<Stmt> {
        // NOT x -> h
        if self.stmt_not.is_match(line) {
            let captures = self.stmt_not.captures(line).expect("regex lib broken");
            let (val, assignee) = (
                captures.get(1).expect("capture 1").as_str(),
                captures.get(2).expect("capture 2").as_str(),
            );
            let val: Value = val.into();
            let dependencies = Dependencies::from_values(vec![&val]);
            let stmt = SetNot(assignee.to_string(), val, dependencies);
            Some(stmt)
        } else {
            None
        }
    }

    // We assume since this is a circuit that each value can only be assigned once
    fn process_stmt(slots: &mut Slots, stmt: &Stmt) {
        match stmt {
            Assign(assignee, val, ..) => {
                if slots.get(assignee).is_none() {
                    slots.insert(assignee.clone(), Circuit::resolve_val(&slots, val));
                }
            }
            AssignExpr(assignee, val1, op, val2, ..) => {
                if slots.get(assignee).is_none() {
                    let val1 = Circuit::resolve_val(&slots, val1);
                    let val2 = Circuit::resolve_val(&slots, val2);
                    let result = match op {
                        And => val1 & val2,
                        Or => val1 | val2,
                        LShift => val1 << val2,
                        RShift => val1 >> val2,
                        Not => panic!("NOT is not part of an AssignExpr"),
                    };
                    slots.insert(assignee.clone(), result);
                }
            }
            SetNot(assignee, val, ..) => {
                if slots.get(assignee).is_none() {
                    slots.insert(assignee.clone(), !Circuit::resolve_val(&slots, val));
                }
            }
        }
    }

    fn resolve_val(slots: &Slots, val: &Value) -> SlotValue {
        match val {
            Literal(x) => *x,
            Slot(key) => *slots
                .get(key)
                .expect(&format!("Unable to resolve slot {:?}", &key)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key.to_string(), $value);
            )+
            m
        }
        };
    );

    const SAMPLE_PROGRAM: &str = "\
        \x20123 -> x\n\
        \x20456 -> y\n\
        \x20x AND y -> d\n\
        \x20x OR y -> e\n\
        \x20x LSHIFT 2 -> f\n\
        \x20y RSHIFT 2 -> g\n\
        \x20NOT x -> h\n\
        \x20NOT y -> i\
        ";

    const SAMPLE_PROGRAM_UNORDERED: &str = "\
        \x20x AND y -> d\n\
        \x20123 -> x\n\
        \x20456 -> y\n\
        \x20x OR y -> e\n\
        \x20x LSHIFT 2 -> f\n\
        \x20y RSHIFT 2 -> g\n\
        \x20NOT x -> h\n\
        \x20NOT y -> i\
        ";

    #[test]
    fn process_stmt() {
        let mut slots = HashMap::new();
        Circuit::process_stmt(
            &mut slots,
            &Assign("x".to_string(), Literal(123), Dependencies::default()),
        );
        assert_eq!(slots, map!("x" => 123));

        Circuit::process_stmt(
            &mut slots,
            &Assign("y".to_string(), Literal(456), Dependencies::default()),
        );
        assert_eq!(slots, map!("x" => 123, "y" => 456));

        Circuit::process_stmt(
            &mut slots,
            &AssignExpr(
                "d".to_string(),
                Slot("x".to_string()),
                And,
                Slot("y".to_string()),
                Dependencies::default(),
            ),
        );
        assert_eq!(slots, map!("x" => 123, "y" => 456, "d" => 72));

        Circuit::process_stmt(
            &mut slots,
            &AssignExpr(
                "f".to_string(),
                Slot("x".to_string()),
                LShift,
                Literal(2),
                Dependencies::default(),
            ),
        );
        assert_eq!(slots, map!("x" => 123, "y" => 456, "d" => 72, "f" => 492));
        Circuit::process_stmt(
            &mut slots,
            &SetNot(
                "h".to_string(),
                Slot("x".to_string()),
                Dependencies::default(),
            ),
        );
        println!("{:#?}", slots);

        assert_eq!(
            slots,
            map!("x" => 123, "y" => 456, "d" => 72, "f" => 492, "h" => 65412 )
        );
    }

    #[test]
    fn extract_operations() {
        let circuit = Circuit::from_program(true, SAMPLE_PROGRAM);
        assert_eq!(
            circuit.statements,
            vec![
                Assign("x".to_string(), Literal(123), Dependencies::default()),
                Assign("y".to_string(), Literal(456), Dependencies::default()),
                AssignExpr(
                    "d".to_string(),
                    Slot("x".to_string()),
                    And,
                    Slot("y".to_string()),
                    Dependencies(vec!["x".to_string(), "y".to_string()]),
                ),
                AssignExpr(
                    "e".to_string(),
                    Slot("x".to_string()),
                    Or,
                    Slot("y".to_string()),
                    Dependencies(vec!["x".to_string(), "y".to_string()]),
                ),
                AssignExpr(
                    "f".to_string(),
                    Slot("x".to_string()),
                    LShift,
                    Literal(2),
                    Dependencies(vec!["x".to_string()]),
                ),
                AssignExpr(
                    "g".to_string(),
                    Slot("y".to_string()),
                    RShift,
                    Literal(2),
                    Dependencies(vec!["y".to_string()]),
                ),
                SetNot(
                    "h".to_string(),
                    Slot("x".to_string()),
                    Dependencies(vec!["x".to_string()]),
                ),
                SetNot(
                    "i".to_string(),
                    Slot("y".to_string()),
                    Dependencies(vec!["y".to_string()]),
                ),
            ]
        );
    }

    #[test]
    fn run_program() {
        let circuit = Circuit::from_program(true, SAMPLE_PROGRAM);
        let slots = circuit.run();
        assert_eq!(
            slots,
            map!(
                "d" => 72,
                "e" => 507,
                "f" => 492,
                "g" => 114,
                "h" => 65412,
                "i" => 65079,
                "x" => 123,
                "y" => 456
            )
        );
    }

    #[test]
    fn run_program_unordered() {
        let circuit = Circuit::from_program(true, SAMPLE_PROGRAM_UNORDERED);
        let slots = circuit.run();
        assert_eq!(
            dbg!(slots),
            map!(
                "d" => 72,
                "e" => 507,
                "f" => 492,
                "g" => 114,
                "h" => 65412,
                "i" => 65079,
                "x" => 123,
                "y" => 456
            )
        );
    }
}
