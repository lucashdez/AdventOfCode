fn read_file_filter(filename: String) -> Vec<String>
{
    let content =
        std::fs::read_to_string(format!("input/{filename}")).expect("SHOULD HAVE READ FILE");
    let splitted = content.split('\n');
    let filtered: Vec<&str> = splitted.filter(|x| x != &"").collect();
    let result = filtered.into_iter().map(|x| x.to_string()).collect();
    result
}

fn points(rival: &str, mychoice: &str) -> i32
{
    let mut points = 0;
    match mychoice
    {
        "X" =>
        {
            points += 1;
            if rival == "A"
            {
                points += 3;
            }
            else if rival == "B"
            {
                points += 0;
            }
            else
            {
                points += 6;
            }
        }
        "Y" =>
        {
            points += 2;
            if rival == "A"
            {
                points += 6;
            }
            else if rival == "B"
            {
                points += 3;
            }
            else
            {
                points += 0;
            }
        }
        "Z" =>
        {
            points += 3;
            if rival == "A"
            {
                points += 0;
            }
            else if rival == "B"
            {
                points += 6;
            }
            else
            {
                points += 3;
            }
        }
        _ =>
        {
            print!("wtf");
            return 0;
        }
    }
    return points;
}

fn p1(content: &Vec<String>) -> i32
{
    let mut final_points = 0;
    for line in content
    {
        let fixedline = line.replace("\r", "");
        let rps_match: Vec<&str> = fixedline.split(' ').collect();
        let rival = rps_match[0];
        let my_choice = rps_match[1];
        final_points += points(rival, my_choice);
    }
    return final_points;
}

fn p2() -> () {}

fn main()
{
    let content = read_file_filter("input.txt".to_string());
    let p1_result = p1(&content);
    println!("P1: {}", p1_result);
}
