use regex::Regex;
use std::collections::HashSet;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<_> = std::fs::read_to_string("input/day07_1.in")?
        .lines()
        .map(|s| s.to_string())
        .collect();
    let cd_re = Regex::new(r"\$ cd ([\w|\.]+)")?;
    let ls_re = Regex::new(r"\$ ls")?;
    let file_re = Regex::new(r"(\d+) ([\w|.|_]+)")?;
    let dir_re = Regex::new(r"dir ([\w|.|_]+)")?;

    let mut tree: Vec<(String, usize)> = Vec::new();
    let mut papa: Vec<usize> = Vec::new();
    let mut kids: Vec<HashSet<usize>> = Vec::new();

    tree.push(("/".to_string(), 0));
    papa.push(0);
    kids.push(HashSet::new());

    let mut curr_dir_ind = 0;
    let mut i = 1;
    let mut line;
    while i < lines.len() {
        line = &lines[i];
        if let Some(cd) = cd_re.captures(line) {
            if cd[1] == *".." {
                curr_dir_ind = papa[curr_dir_ind];
            } else {
                for kid in kids[curr_dir_ind].iter() {
                    if tree[*kid].0 == cd[1].to_string() {
                        curr_dir_ind = *kid;
                        break;
                    }
                }
            }
            i += 1;
        } else if let Some(_) = ls_re.captures(line) {
            i += 1;
            line = &lines[i];
            while i < lines.len() && line.chars().next().unwrap() != '$' {
                if let Some(file_cap) = file_re.captures(line) {
                    let ind = tree.len();
                    tree.push((file_cap[2].to_string(), file_cap[1].parse()?));
                    papa.push(curr_dir_ind);
                    kids[curr_dir_ind].insert(ind);
                    kids.push(HashSet::new());
                } else if let Some(dir_cap) = dir_re.captures(line) {
                    let ind = tree.len();
                    tree.push((dir_cap[1].to_string(), 0));
                    papa.push(curr_dir_ind);
                    kids[curr_dir_ind].insert(ind);
                    kids.push(HashSet::new());
                }
                i += 1;
                if i < lines.len() {
                    line = &lines[i];
                }
            }
        }
    }

    calc_size(&mut tree, &kids, 0);
    let needed_space = 30000000 - (70000000 - tree[0].1);

    let mut res = tree[0].clone();
    for (i, node) in tree.iter().enumerate() {
        if !kids[i].is_empty() && node.1 < res.1 && node.1 >= needed_space {
            res = node.clone()
        }
    }
    println!(
        "The biggest directory is '{}', with a size of {}",
        res.0, res.1
    );

    Ok(())
}

fn calc_size(tree: &mut Vec<(String, usize)>, kids: &Vec<HashSet<usize>>, curr_node: usize) {
    for kid in kids[curr_node].iter() {
        calc_size(tree, kids, *kid);
        tree[curr_node].1 += tree[*kid].1;
    }
}
