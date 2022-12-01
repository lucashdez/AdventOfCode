use std::{collections::HashMap, fs};
fn read_file_filter(filename: String) -> Vec<String>
{
    let content = fs::read_to_string(format!("input/{filename}")).expect("SHOULD HAVE READ FILE");
    let splitted = content.split('\n');
    let filtered: Vec<&str> = splitted.filter(|x| x != &"").collect();
    let result = filtered.into_iter().map(|x| x.to_string()).collect();
    result
}

fn p1(
    elf_hash: &mut HashMap<i32, Vec<String>>,
    elf_sum: &mut Vec<i32>,
    content: &Vec<String>,
) -> i32
{
    let mut max: i32 = 0;
    let mut elf_identifier = 0;
    for line in content
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
    for key in elf_hash.keys()
    {
        let mut sum = 0;
        for val in elf_hash.get(key).unwrap()
        {
            sum += val.parse::<i32>().unwrap();
        }

        elf_sum.push(sum)
    }
    for v in elf_sum.clone()
    {
        if v >= max
        {
            max = v;
        }
    }
    max
}

fn p2(elf_sum: &mut Vec<i32>) -> i32
{
    elf_sum.sort();
    let result =
        elf_sum[elf_sum.len() - 1] + elf_sum[elf_sum.len() - 2] + elf_sum[elf_sum.len() - 3];
    result
}

fn main()
{
    let content = read_file_filter("input.txt".to_string());
    let mut elf_hash: HashMap<i32, Vec<String>> = HashMap::new();
    let mut elf_sum: Vec<i32> = Vec::new();
    let p1_result = p1(&mut elf_hash, &mut elf_sum, &content);
    println!("P1: {}", p1_result);
    let p2_result = p2(&mut elf_sum);
    println!("P2: {}", p2_result);

    // TODO : Get rid of the hardcoding
}
