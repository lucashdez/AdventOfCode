use std::{collections::HashMap, fs};
fn main()
{
    // TODO : Split in functions
    let content = fs::read_to_string("input/input.txt").expect("SHOULD HAVE READ FILE");
    let splitted = content.split('\n');
    let filtered = splitted.filter(|x| x != &"");

    let mut elf_hash: HashMap<i32, Vec<String>> = HashMap::new();
    let mut elf_identifier = 0;
    for line in filtered
    {
        if line == "\r"
        {
            elf_identifier += 1;
            continue;
        }
        let fixedline = line.replace("\r", "");
        if elf_hash.contains_key(&elf_identifier)
        {
            elf_hash
                .get_mut(&elf_identifier)
                .as_mut()
                .unwrap()
                .push(fixedline.to_string());
        }
        else
        {
            elf_hash.insert(elf_identifier, vec![fixedline.to_string()]);
        }
    }
    let mut elf_sum: Vec<(i32, i32)> = Vec::new();

    for key in elf_hash.keys()
    {
        let k: i32 = key.to_owned();
        let mut sum = 0;
        for val in elf_hash.get(key).unwrap()
        {
            sum += val.parse::<i32>().unwrap();
        }

        elf_sum.push((k, sum))
    }
    // TODO : Get rid of the hardcoding
    let mut max = 0;
    for (_k, v) in elf_sum.clone()
    {
        if v >= max
        {
            max = v;
        }
    }
    println!("P1: {}", max);
    let mut second = 0;
    let mut third = 0;
    for (_k, v) in elf_sum.clone()
    {
        if v >= second && v != max
        {
            second = v;
        }
    }
    for (_k, v) in elf_sum.clone()
    {
        if v >= third && v != max && v != second
        {
            third = v;
        }
    }
    println!("P2: {}", max + second + third);
}
