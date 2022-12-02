fn read_file_filter(filename: String) -> Vec<String>
{
    let content =
        std::fs::read_to_string(format!("input/{filename}")).expect("SHOULD HAVE READ FILE");
    let splitted = content.split('\n');
    let filtered: Vec<&str> = splitted.filter(|x| x != &"").collect();
    let result = filtered.into_iter().map(|x| x.to_string()).collect();
    result
}

fn p1() -> () {}

fn p2() -> () {}

fn main() {}
