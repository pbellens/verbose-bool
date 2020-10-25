mod condition;

use condition::Condition;

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
