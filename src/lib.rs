use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug, Clone)]
pub struct Value {
    value: u16,
    from: Option<Rc<RefCell<Op>>>,
    should_have_brackets: bool,
}

impl Value {
    pub fn show(&mut self) -> String {
        recursive_remove_brackets(self);
        format!("{}", self)
    }
}

fn recursive_remove_brackets(root: &mut Value) {
    root.should_have_brackets = false;

    fn inner(node: &mut Value) {
        let Some(from) = &mut node.from else {
            node.should_have_brackets = false;
            return;
        };

        let mut from = from.borrow_mut();
        let mut should_left_have_brackets = false;
        let mut should_right_have_brackets = false;
        from.with_children(|left, right| {
            if let Some(left_op) = left.from.as_ref() {
                let left_op = left_op.borrow_mut();
                if from.should_left_child_have_brackets(&left_op) {
                    should_left_have_brackets = true;
                }
            }
            if let Some(right_op) = right.from.as_ref() {
                let right_op = right_op.borrow_mut();
                if from.should_right_child_have_brackets(&right_op) {
                    should_right_have_brackets = true;
                }
            }
        });
        from.with_children_mut(|left, right| {
            left.should_have_brackets = should_left_have_brackets;
            right.should_have_brackets = should_right_have_brackets;
            inner(left);
            inner(right);
        });
    }

    inner(root)
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value {
            value,
            from: None,
            should_have_brackets: true,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(from) = &self.from {
            if self.should_have_brackets {
                write!(f, "({})", RefCell::borrow(from))
            } else {
                write!(f, "{}", RefCell::borrow(from))
            }
        } else {
            write!(f, "{}", self.value)
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add(a, b) => write!(f, "{} + {}", a, b),
            Op::Sub(a, b) => write!(f, "{} - {}", a, b),
            Op::Mul(a, b) => write!(f, "{} * {}", a, b),
            Op::Div(a, b) => write!(f, "{} / {}", a, b),
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

impl Op {
    fn priority(&self) -> u8 {
        match self {
            Op::Add(_, _) => 1,
            Op::Sub(_, _) => 1,
            Op::Mul(_, _) => 2,
            Op::Div(_, _) => 2,
        }
    }

    fn should_left_child_have_brackets(&self, left_child: &Self) -> bool {
        self.priority() > left_child.priority()
    }

    fn should_right_child_have_brackets(&self, right_child: &Self) -> bool {
        match (self, right_child) {
            (Op::Add(_, _), Op::Add(_, _)) => false,
            (Op::Add(_, _), Op::Sub(_, _)) => false,
            (Op::Add(_, _), Op::Mul(_, _)) => false,
            (Op::Add(_, _), Op::Div(_, _)) => false,
            (Op::Sub(_, _), Op::Mul(_, _)) => false,
            (Op::Sub(_, _), Op::Div(_, _)) => false,
            (Op::Mul(_, _), Op::Mul(_, _)) => false,
            (Op::Mul(_, _), Op::Div(_, _)) => false,
            (Op::Sub(_, _), Op::Add(_, _)) => true,
            (Op::Sub(_, _), Op::Sub(_, _)) => true,
            (Op::Mul(_, _), Op::Add(_, _)) => true,
            (Op::Mul(_, _), Op::Sub(_, _)) => true,
            (Op::Div(_, _), Op::Add(_, _)) => true,
            (Op::Div(_, _), Op::Sub(_, _)) => true,
            (Op::Div(_, _), Op::Mul(_, _)) => true,
            (Op::Div(_, _), Op::Div(_, _)) => true,
        }
    }

    fn with_children(&self, f: impl FnOnce(&Value, &Value)) {
        match self {
            Op::Add(a, b) => f(a, b),
            Op::Sub(a, b) => f(a, b),
            Op::Mul(a, b) => f(a, b),
            Op::Div(a, b) => f(a, b),
        }
    }

    fn with_children_mut(&mut self, f: impl FnOnce(&mut Value, &mut Value)) {
        match self {
            Op::Add(a, b) => f(a, b),
            Op::Sub(a, b) => f(a, b),
            Op::Mul(a, b) => f(a, b),
            Op::Div(a, b) => f(a, b),
        }
    }
}

fn add(a: &Value, b: &Value) -> Option<Value> {
    Some(Value {
        value: a.value.checked_add(b.value)?,
        from: Some(Rc::new(RefCell::new(Op::Add(a.clone(), b.clone())))),
        should_have_brackets: true,
    })
}

fn sub(a: &Value, b: &Value) -> Option<Value> {
    Some(Value {
        value: a.value.checked_sub(b.value)?,
        from: Some(Rc::new(RefCell::new(Op::Sub(a.clone(), b.clone())))),
        should_have_brackets: true,
    })
}

fn mul(a: &Value, b: &Value) -> Option<Value> {
    Some(Value {
        value: a.value.checked_mul(b.value)?,
        from: Some(Rc::new(RefCell::new(Op::Mul(a.clone(), b.clone())))),
        should_have_brackets: true,
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
        from: Some(Rc::new(RefCell::new(Op::Div(a.clone(), b.clone())))),
        should_have_brackets: true,
    })
}

fn try_solve_list(
    target: u16,
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

fn try_solve_list_all(
    target: u16,
    list: &mut Vec<Value>,
    a: &Value,
    b: &Value,
    f: impl FnOnce(&Value, &Value) -> Option<Value>,
    ans: &mut Vec<Value>,
) {
    let Some(new_value) = (f)(a, b) else {
        return;
    };
    list.push(new_value);
    solve_inner_all(target, &*list, ans);
    list.pop();
}

fn solve_inner_all(target: u16, list: &[Value], all_ans: &mut Vec<Value>) {
    if list.len() == 1 {
        if list[0].value == target {
            all_ans.push(list[0].clone());
        }

        return;
    }

    let mut new_list = list.to_vec();
    new_list.sort_unstable_by_key(|x: &Value| u16::MAX - x.value);
    for i in 0..list.len() - 1 {
        if i > 0 && new_list[i].value == new_list[i - 1].value {
            continue;
        }

        let item_i = new_list.remove(i);
        for j in i + 1..list.len() {
            if j > i + 1 && new_list[j - 1].value == new_list[j - 2].value {
                continue;
            }

            let item_j = new_list.remove(j - 1);
            try_solve_list_all(target, &mut new_list, &item_i, &item_j, add, all_ans);
            try_solve_list_all(target, &mut new_list, &item_i, &item_j, sub, all_ans);
            try_solve_list_all(target, &mut new_list, &item_i, &item_j, mul, all_ans);
            try_solve_list_all(target, &mut new_list, &item_i, &item_j, div, all_ans);
            new_list.insert(j - 1, item_j);
        }

        new_list.insert(i, item_i);
    }
}

fn solve_inner(target: u16, list: &[Value]) -> Option<Value> {
    if list.len() == 1 {
        if list[0].value == target {
            return Some(list[0].clone());
        }

        return None;
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

pub fn solve_list(target: u16, list: &[u16]) -> Option<Value> {
    let list: Vec<Value> = list.iter().map(|x| (*x).into()).collect::<Vec<_>>();
    solve_inner(target, &list)
}

pub fn solve_list_all(target: u16, list: &[u16]) -> Vec<Value> {
    let mut ans = Vec::new();
    let list: Vec<Value> = list.iter().map(|x| (*x).into()).collect::<Vec<_>>();
    solve_inner_all(target, &list, &mut ans);
    ans
}

pub fn solve_24(a: u16, b: u16, c: u16, d: u16) -> Option<Value> {
    solve_list(24, &[a, b, c, d])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_all() {
        println!(
            "{}",
            solve_list_all(24, &[8, 1, 11, 9])
                .into_iter()
                .map(|mut x| x.show())
                .collect::<Vec<_>>()
                .join(", \n")
        );
    }

    #[test]
    fn test_brackets() {
        let mut value: Value = sub(&8.into(), &add(&1.into(), &3.into()).unwrap()).unwrap();
        assert_eq!(&value.show(), "8 - (1 + 3)");
        let mut value: Value = div(&8.into(), &add(&1.into(), &3.into()).unwrap()).unwrap();
        assert_eq!(&value.show(), "8 / (1 + 3)");
        let mut value: Value = add(&8.into(), &add(&1.into(), &3.into()).unwrap()).unwrap();
        assert_eq!(&value.show(), "8 + 1 + 3");
        let mut value: Value = mul(&8.into(), &add(&1.into(), &3.into()).unwrap()).unwrap();
        assert_eq!(&value.show(), "8 * (1 + 3)");
    }

    #[test]
    fn it_works() {
        println!("{}", solve_24(9, 2, 7, 6).unwrap().show());
        println!("{}", solve_24(7, 7, 2, 1).unwrap().show());
        println!("{}", solve_24(9, 9, 8, 3).unwrap().show());
        println!("{}", solve_24(11, 12, 13, 9).unwrap().show());
        println!("{}", solve_24(11, 12, 13, 9).unwrap().show());
        assert!(solve_24(6, 3, 3, 4).is_some());
        assert!(solve_24(7, 7, 5, 5).is_some());
        assert!(solve_24(7, 7, 25, 1).is_some());
        assert!(solve_24(2, 2, 2, 2).is_none());
        assert!(solve_24(1, 1, 1, 1).is_none());
    }
}
