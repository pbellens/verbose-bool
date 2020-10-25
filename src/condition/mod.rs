mod truth;

use truth::Truth;

pub struct Condition {
    pub name: String,
    expr: Box<dyn Fn(String) -> Truth>,
}

impl Condition {
    pub fn eval(&self) -> Truth {
        (*self.expr)(self.name.to_string()) 
    }

    pub fn new(n: &str, e: Box<dyn Fn() -> bool>) -> Condition {
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

    pub fn and(self, other: Condition) -> Condition {
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

    pub fn or(self, other: Condition) -> Condition {
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

    pub fn not(c: Condition) -> Condition {
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

