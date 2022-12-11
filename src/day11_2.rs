use regex::Regex;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test: Test,
}

impl Monkey {
    fn new(items: Vec<usize>, op: Operation, test: Test) -> Self {
        Monkey { items, op, test }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Addition(usize),
    Multiplication(usize),
    Exponentiation,
}

#[derive(Debug, Clone)]
struct Test {
    test_val: usize,
    true_monkey: usize,
    false_monkey: usize,
}
impl Test {
    fn new(test: usize, true_monkey: usize, false_monkey: usize) -> Self {
        Test {
            test_val: test,
            true_monkey,
            false_monkey,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut monkeys = read()?;
    let mut monkey_cnt: Vec<usize> = (0..monkeys.len()).map(|_| 0).collect();

    let max_test_val = monkeys.iter().map(|m| m.test.test_val).product::<usize>();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            for item in monkey.items {
                let new_stress = match monkey.op {
                    Operation::Addition(val) => item + val,
                    Operation::Multiplication(val) => item * val,
                    Operation::Exponentiation => item * item,
                } % max_test_val;

                if new_stress % monkey.test.test_val == 0 {
                    monkeys[monkey.test.true_monkey].items.push(new_stress);
                } else {
                    monkeys[monkey.test.false_monkey].items.push(new_stress);
                }
                monkey_cnt[i] += 1;
            }
            monkeys[i].items = vec![];
        }
    }

    monkey_cnt.sort();
    print!(
        "The answer is: {}",
        monkey_cnt.iter().rev().take(2).product::<usize>()
    );

    Ok(())
}

fn read() -> Result<Vec<Monkey>, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day11_1.in")?;
    let monkey_re = Regex::new(
        r"(Monkey \d:
  Starting items: .*
  Operation: new = .*
  Test: divisible by .*
    If true: throw to monkey .*
    If false: throw to monkey .*)",
    )?;

    let test_re = Regex::new(
        r"  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)",
    )?;

    let op_add_re = Regex::new(r"  Operation: new = old \+ (\d+)")?;
    let op_mul_re = Regex::new(r"  Operation: new = old \* (\d+)")?;

    let items_re = Regex::new(r"  Starting items: (.*)")?;
    let item_re = Regex::new(r"(\d+)")?;

    let mut res = vec![];

    for c in monkey_re.captures_iter(&input) {
        let monkey_text = &c[0];
        let test_caps = test_re.captures(monkey_text).unwrap();
        let test = Test::new(
            test_caps[1].parse().unwrap(),
            test_caps[2].parse().unwrap(),
            test_caps[3].parse().unwrap(),
        );

        let op = if let Some(caps) = op_add_re.captures(monkey_text) {
            Operation::Addition(caps[1].parse().unwrap())
        } else if let Some(caps) = op_mul_re.captures(monkey_text) {
            Operation::Multiplication(caps[1].parse().unwrap())
        } else {
            Operation::Exponentiation
        };

        let items_text = &items_re.captures(monkey_text).unwrap()[1];
        let items = item_re
            .captures_iter(&items_text)
            .map(|cap| cap[1].parse().unwrap())
            .collect();

        res.push(Monkey::new(items, op, test));
    }

    Ok(res)
}
