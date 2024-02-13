use std::{fmt::Display, rc::Rc};

#[derive(Debug, Clone)]
pub struct Value {
    value: u8,
    from: Option<Rc<Op>>,
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value { value, from: None }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(from) = &self.from {
            write!(f, "{}", from)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add(a, b) => write!(f, "({} + {})", a, b),
            Op::Sub(a, b) => write!(f, "({} - {})", a, b),
            Op::Mul(a, b) => write!(f, "({} * {})", a, b),
            Op::Div(a, b) => write!(f, "({} / {})", a, b),
        }
    }
}

fn add(a: &Value, b: &Value) -> Option<Value> {
    Some(Value {
        value: a.value.checked_add(b.value)?,
        from: Some(Rc::new(Op::Add(a.clone(), b.clone()))),
    })
}

fn sub(a: &Value, b: &Value) -> Option<Value> {
    Some(Value {
        value: a.value.checked_sub(b.value)?,
        from: Some(Rc::new(Op::Sub(a.clone(), b.clone()))),
    })
}

fn mul(a: &Value, b: &Value) -> Option<Value> {
    Some(Value {
        value: a.value.checked_mul(b.value)?,
        from: Some(Rc::new(Op::Mul(a.clone(), b.clone()))),
    })
}

fn div(a: &Value, b: &Value) -> Option<Value> {
    if b.value == 0 {
        return None;
    }

    if a.value % b.value != 0 {
        return None;
    }

    if a.value < b.value {
        return None;
    }

    Some(Value {
        value: a.value / b.value,
        from: Some(Rc::new(Op::Div(a.clone(), b.clone()))),
    })
}

fn try_solve_list(
    target: u8,
    list: &mut Vec<Value>,
    a: &Value,
    b: &Value,
    f: impl FnOnce(&Value, &Value) -> Option<Value>,
) -> Option<Value> {
    let new_value = (f)(a, b)?;
    list.push(new_value);
    if let Some(ans) = solve_inner(target, &*list) {
        return Some(ans);
    }

    list.pop();
    None
}

fn solve_inner(target: u8, list: &[Value]) -> Option<Value> {
    if list.len() == 1 {
        if list[0].value == target {
            return Some(list[0].clone());
        } else {
            return None;
        }
    }

    let mut new_list = list.to_vec();
    new_list.sort_unstable_by_key(|x: &Value| 255 - x.value);
    for i in 0..list.len() - 1 {
        let item_i = new_list.remove(i);
        for j in i + 1..list.len() {
            let item_j = new_list.remove(j - 1);
            if let Some(ans) = try_solve_list(target, &mut new_list, &item_i, &item_j, add) {
                return Some(ans);
            }
            if let Some(ans) = try_solve_list(target, &mut new_list, &item_i, &item_j, sub) {
                return Some(ans);
            }
            if let Some(ans) = try_solve_list(target, &mut new_list, &item_i, &item_j, mul) {
                return Some(ans);
            }
            if let Some(ans) = try_solve_list(target, &mut new_list, &item_i, &item_j, div) {
                return Some(ans);
            }

            new_list.insert(j - 1, item_j);
        }

        new_list.insert(i, item_i);
    }

    None
}

pub fn solve_list(target: u8, list: &[u8]) -> Option<Value> {
    let list: Vec<Value> = list.iter().map(|x| (*x).into()).collect::<Vec<_>>();
    solve_inner(target, &list)
}

pub fn solve(target: u8, a: u8, b: u8, c: u8, d: u8) -> Option<Value> {
    let list = vec![a.into(), b.into(), c.into(), d.into()];
    solve_inner(target, &list)
}

pub fn solve_24(a: u8, b: u8, c: u8, d: u8) -> Option<Value> {
    solve(24, a, b, c, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(solve_24(6, 3, 3, 4).is_some());
        assert!(solve_24(7, 7, 5, 5).is_some());
        assert!(solve_24(7, 7, 25, 1).is_some());
        println!("{}", solve_24(7, 7, 2, 1).unwrap());
        println!("{}", solve_24(9, 9, 8, 3).unwrap());
        println!("{}", solve_24(9, 2, 7, 6).unwrap());
        println!("{}", solve_24(9, 9, 8, 3).unwrap());
        println!("{}", solve_24(11, 12, 13, 9).unwrap());
        assert!(solve_24(2, 2, 2, 2).is_none());
        assert!(solve_24(1, 1, 1, 1).is_none());
    }
}
