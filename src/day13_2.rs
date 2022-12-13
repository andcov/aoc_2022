use std::cmp::Ordering;
use std::str::{Chars, FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
struct List {
    val: Option<u32>,
    vec: Option<Vec<List>>,
}

fn build_recurse(chars: &mut Chars) -> Result<List, Box<dyn std::error::Error>> {
    let mut list = List {
        val: None,
        vec: Some(Vec::new()),
    };

    while let Some(ch) = chars.next() {
        if ch == '[' {
            let sub_list = build_recurse(chars)?;
            list.vec.as_mut().unwrap().push(sub_list);
        } else if ch == ']' {
            return Ok(list);
        } else if ch == ',' {
            continue;
        } else {
            let mut new_chars = chars.clone();
            let mut num = ch.to_digit(10).unwrap();
            while let Some(ch) = new_chars.next() {
                if !ch.is_digit(10) {
                    break;
                }
                num = num * 10 + ch.to_digit(10).unwrap();
            }
            let sub_list = List {
                val: Some(num),
                vec: None,
            };
            list.vec.as_mut().unwrap().push(sub_list);
        }
    }

    Ok(list)
}

impl FromStr for List {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        chars.next();
        build_recurse(&mut chars)
    }
}

impl ToString for List {
    fn to_string(&self) -> String {
        match self.val {
            Some(v) => v.to_string(),
            None => {
                let vec = self.vec.as_ref().unwrap();
                let mut res = "[".to_string();
                for l in vec {
                    let s = l.to_string();
                    res.push_str(&s);
                    res.push(',');
                }
                if vec.len() > 0 {
                    res.pop();
                }
                res.push(']');
                res
            }
        }
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.val.is_some() && other.val.is_some() {
            return self.val.cmp(&other.val);
        } else if self.val.is_some() && other.vec.is_some() {
            let new_list = List {
                val: None,
                vec: Some(vec![self.clone()]),
            };
            return new_list.cmp(&other);
        } else if self.vec.is_some() && other.val.is_some() {
            let new_list = List {
                val: None,
                vec: Some(vec![other.clone()]),
            };
            return self.cmp(&new_list);
        } else {
            if self.vec.as_ref().unwrap().len() == 0 && other.vec.as_ref().unwrap().len() != 0 {
                return Ordering::Less;
            }

            for i in 0..self.vec.as_ref().unwrap().len() {
                if other.vec.as_ref().unwrap().len() <= i {
                    return Ordering::Greater;
                }
                match self.vec.as_ref().unwrap()[i].cmp(&other.vec.as_ref().unwrap()[i]) {
                    Ordering::Less | Ordering::Greater => {
                        return self.vec.as_ref().unwrap()[i].cmp(&other.vec.as_ref().unwrap()[i])
                    }
                    _ => continue,
                }
            }

            if self.vec.as_ref().unwrap().len() < other.vec.as_ref().unwrap().len() {
                return Ordering::Less;
            }
            return Ordering::Equal;
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day13_2.in")?;
    let mut packets = vec![];
    packets.push(List::from_str("[[2]]")?);
    packets.push(List::from_str("[[6]]")?);

    for l in input.lines() {
        if l == "" {
            continue;
        }
        packets.push(List::from_str(l)?);
    }
    packets.sort();

    let val = packets
        .iter()
        .enumerate()
        .filter(|pack| -> bool {
            *pack.1 == List::from_str("[[2]]").unwrap()
                || *pack.1 == List::from_str("[[6]]").unwrap()
        })
        .map(|(i, _)| i + 1)
        .product::<usize>();

    println!("The answer is: {}", val);
    Ok(())
}
