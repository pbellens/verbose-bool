struct Truth {
    value: bool,
    trace: Option<Vec<String>>
}

impl Truth {
    fn new(v: bool) -> Self {
        Truth {
            value: v,
            trace: None
        }
    }
}

struct Condition {
    name: String,
    expr: Box<dyn Fn() -> Truth>,
}

impl Condition {
    fn eval(&self) -> Truth {
        (*self.expr)() 
    }

    fn new(n: String, e: Box<dyn Fn() -> bool>) -> Condition {
        Condition {
            name: n.to_string(),
            expr: Box::new(move || {
                match e()
                {
                    true => Truth::new(true),
                    false => Truth {
                        value: false,
                        trace: Some(vec![n.to_string()])
                    }
                }})
        }
    }

    fn and(self, other: Condition) -> Condition {
        Condition {
            name: format!("{} and {}", self.name, other.name),
            expr: Box::new(move || {
                if let Truth { value: false, trace: Some(mut t) } = self.eval() {
                    t.push(format!("{} and {}", self.name, other.name));
                    return Truth {
                        value: false,
                        trace: Some(t)
                    }
                }

                if let Truth { value: false, trace: Some(mut t) } = other.eval() {
                    t.push(format!("{} and {}", self.name, other.name));
                    return Truth {
                        value: false,
                        trace: Some(t)
                    }
                }

                Truth::new(true)
            })
        }
    }
}

fn main() {
    let t1 = Condition::new("A".to_string(), Box::new(|| true));
    let t2 = Condition::new("B".to_string(), Box::new(|| true));
    let t3 = Condition::new("C".to_string(), Box::new(|| false));
    let t4 = Condition::new("D".to_string(), Box::new(|| true));

    //let c1 = t3.and(t4);
    //let c2 = t1.and(t2);
    let c = t1.and(t2).and(t3).and(t4);

    let value = c.eval();
    println!("evaluated \"{}\" to {:?}", c.name, value.value);
    match &value.trace {
        None => println!("found no trace"),
        Some(t) => {
            for l in t.iter().rev() {
                println!("{}", l);
            }
        }
    }
}
