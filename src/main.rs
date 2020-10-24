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
    expr: Box<dyn Fn(String) -> Truth>,
}

impl Condition {
    fn eval(&self) -> Truth {
        (*self.expr)(self.name.to_string()) 
    }

    fn new(n: &str, e: Box<dyn Fn() -> bool>) -> Condition {
        Condition {
            name: n.to_string(),
            expr: Box::new(move |n| {
                match e()
                {
                    true => Truth::new(true),
                    false => Truth {
                        value: false,
                        trace: Some(vec![n])
                    }
                }})
        }
    }

    fn and(self, other: Condition) -> Condition {
        Condition {
            name: format!("({} and {})", self.name, other.name),
            expr: Box::new(move |n| {
                if let Truth { value: false, trace: Some(mut t) } = self.eval() {
                    t.push(n);
                    return Truth {
                        value: false,
                        trace: Some(t)
                    }
                }

                if let Truth { value: false, trace: Some(mut t) } = other.eval() {
                    t.push(n);
                    return Truth {
                        value: false,
                        trace: Some(t)
                    }
                }

                Truth::new(true)
            })
        }
    }

    fn or(self, other: Condition) -> Condition {
        Condition {
            name: format!("({} or {})", self.name, other.name),
            expr: Box::new(move |n| {
                if let Truth { value: false, trace: _ } = self.eval() {
                    if let Truth { value: false, trace: Some(mut t) } = other.eval() {
                        t.push(n);
                        return Truth {
                            value: false,
                            trace: Some(t)
                        }
                    }
                }
                Truth::new(true)
            })
        }
    }

    fn not(c: Condition) -> Condition {
        Condition {
            name: format!("(not {})", c.name),
            expr: Box::new(move |n| {
                if let Truth { value: true, trace: _ } = c.eval() {
                    return Truth {
                        value: false,
                        trace: Some(vec![n])
                    }
                }
                Truth::new(true)
            })
        }
    }
}

fn main() {
    let t1 = Condition::new("A", Box::new(|| true));
    let t2 = Condition::new("B", Box::new(|| false));
    let t3 = Condition::new("C", Box::new(|| true));
    let t4 = Condition::new("D", Box::new(|| false));

    let c1 = t3.or(t4);
    let c2 = Condition::not(t1);
    let c = c1.and(c2);

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
