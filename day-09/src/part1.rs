#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Entry {
    File(u16),
    FreeSpace,
}

fn parse(input: &str) -> Vec<Entry> {
    let mut cur_id = 0;
    let mut files = true;
    let mut res = vec![];
    for c in input.chars() {
        let n = c.to_digit(10).unwrap();
        if n == 0 {
            files = !files;
            continue;
        }
        if files {
            let tmp = Entry::File(cur_id);
            for _ in 0..n {
                res.push(tmp);
            }
            cur_id += 1;
        } else {
            for _ in 0..n {
                res.push(Entry::FreeSpace);
            }
        }
        files = !files;
    }

    res
}

pub fn process(input: &str) -> String {
    let mut entries = parse(input);
    let mut l = 0;
    let mut r = entries.len() - 1;

    while r > l {
        if entries[l] != Entry::FreeSpace {
            l += 1;
            continue;
        }
        if entries[r] == Entry::FreeSpace {
            r -= 1;
            continue;
        }
        // entries[l] = Free Space & entries[r] = File
        entries[l] = entries[r];
        entries[r] = Entry::FreeSpace;
        l += 1;
        r -= 1;
    }

    let res: u64 = entries
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| match x {
            Entry::File(id) => Some(i as u64 * id as u64),
            Entry::FreeSpace => None,
        })
        .sum();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input));
    }
}
