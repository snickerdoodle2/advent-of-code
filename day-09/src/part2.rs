#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Entry {
    File(u8, u16),
    FreeSpace(u8),
}

fn parse(input: &str) -> Vec<Entry> {
    let mut cur_id = 0;
    let mut files = true;
    let mut res = vec![];
    for c in input.chars() {
        let n = c.to_digit(10).unwrap() as u8;
        if files {
            res.push(Entry::File(n, cur_id));
            cur_id += 1;
        } else {
            res.push(Entry::FreeSpace(n));
        }
        files = !files;
    }

    res
}

pub fn process(input: &str) -> String {
    let mut entries = parse(input);
    for i in (0..entries.len()).rev() {
        let Entry::File(size, id) = entries[i] else {
            continue;
        };

        for j in 0..i {
            let Entry::FreeSpace(free_space) = entries[j] else {
                continue;
            };

            if size == free_space {
                entries[j] = Entry::File(size, id);
                entries[i] = Entry::FreeSpace(size);
                break;
            } else if size < free_space {
                let remaining = free_space - size;
                entries[j] = Entry::FreeSpace(remaining);
                entries[i] = Entry::FreeSpace(size);
                entries.insert(j, Entry::File(size, id));
                break;
            }
        }
    }

    let (_, res) = entries
        .into_iter()
        .fold((0, 0), |(idx, sum), cur| match cur {
            Entry::File(size, id) => {
                let cur_sum: usize = (idx..(idx + size as usize)).map(|i| i * id as usize).sum();
                (idx + size as usize, sum + cur_sum)
            }
            Entry::FreeSpace(size) => (idx + size as usize, sum),
        });

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input));
    }
}
