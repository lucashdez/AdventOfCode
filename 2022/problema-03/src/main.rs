use std::collections::HashMap;
// {{{ fill_hash
fn fill_points(hash: &mut HashMap<&str, i8>)
{
    hash.insert("a", 1);
    hash.insert("b", 2);
    hash.insert("c", 3);
    hash.insert("d", 4);
    hash.insert("e", 5);
    hash.insert("f", 6);
    hash.insert("g", 7);
    hash.insert("h", 8);
    hash.insert("i", 9);
    hash.insert("j", 10);
    hash.insert("k", 11);
    hash.insert("l", 12);
    hash.insert("m", 13);
    hash.insert("n", 14);
    hash.insert("o", 15);
    hash.insert("p", 16);
    hash.insert("q", 17);
    hash.insert("r", 18);
    hash.insert("s", 19);
    hash.insert("t", 20);
    hash.insert("u", 21);
    hash.insert("v", 22);
    hash.insert("w", 23);
    hash.insert("x", 24);
    hash.insert("y", 25);
    hash.insert("z", 26);
    hash.insert("A", 27);
    hash.insert("B", 28);
    hash.insert("C", 29);
    hash.insert("D", 30);
    hash.insert("E", 31);
    hash.insert("F", 32);
    hash.insert("G", 33);
    hash.insert("H", 34);
    hash.insert("I", 35);
    hash.insert("J", 36);
    hash.insert("K", 37);
    hash.insert("L", 38);
    hash.insert("M", 39);
    hash.insert("N", 40);
    hash.insert("O", 41);
    hash.insert("P", 42);
    hash.insert("Q", 43);
    hash.insert("R", 44);
    hash.insert("S", 45);
    hash.insert("T", 46);
    hash.insert("U", 47);
    hash.insert("V", 48);
    hash.insert("W", 49);
    hash.insert("X", 50);
    hash.insert("Y", 51);
    hash.insert("Z", 52);
}
// }}}

fn read_file_filter(filename: String) -> Vec<String>
{
    let content =
        std::fs::read_to_string(format!("input/{filename}")).expect("SHOULD HAVE READ FILE");
    let splitted = content.split('\n');
    let filtered: Vec<&str> = splitted.filter(|x| x != &"").collect();
    let result = filtered.into_iter().map(|x| x.to_string()).collect();
    result
}

fn p1(content: &Vec<String>, points: &HashMap<&str, i8>) -> i32
{
    let mut result: i32 = 0;
    for line in content
    {
        let tuple: (&str, &str) = line.split_at(line.len() / 2);
        println!("T1: {}\n T2: {}", tuple.0, tuple.1);
        let mut v_0: Vec<&str> = Vec::new();
        let mut v_1: Vec<&str> = Vec::new();
        let chars_0 = tuple.0.chars().as_str().split("");
        let chars_1 = tuple.1.chars().as_str().split("");
        for char in chars_0
        {
            if char == "" || char == "\n" || char == "\r"
            {
                continue;
            }
            if v_0.contains(&char)
            {
                continue;
            }
            v_0.push(char);
        }
        for char in chars_1
        {
            if char == "" || char == "\n" || char == "\r"
            {
                continue;
            }
            if v_1.contains(&char)
            {
                continue;
            }
            v_1.push(char);
        }
        for char in v_0
        {
            if v_1.contains(&char)
            {
                result += *(points.get(char).unwrap()) as i32;
                break;
            }
        }
    }
    return result;
}

fn p2(content: &Vec<String>, points: &HashMap<&str, i8>) -> i32
{
    let mut result = 0;
    return result;
}

fn main()
{
    let content = read_file_filter("input.txt".to_string());
    let mut points: HashMap<&str, i8> = HashMap::new();
    fill_points(&mut points);
    let p1_result = p1(&content, &points);
    println!("P1 result: {}", p1_result);
}
