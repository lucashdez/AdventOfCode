fn read_file_filter(filename: String) -> Vec<String>
{
    let content =
        std::fs::read_to_string(format!("input/{filename}")).expect("SHOULD HAVE READ FILE");
    let splitted = content.split('\n');
    let filtered: Vec<&str> = splitted.filter(|x| x != &"").collect();
    let result = filtered.into_iter().map(|x| x.to_string()).collect();
    result
}
// {{{ points_p1
fn points_p1(rival: &str, mychoice: &str) -> i32
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
// }}}

fn p1(content: &Vec<String>) -> i32
{
    let mut final_points = 0;
    for line in content
    {
        let fixedline = line.replace("\r", "");
        let rps_match: Vec<&str> = fixedline.split(' ').collect();
        let rival = rps_match[0];
        let my_choice = rps_match[1];
        final_points += points_p1(rival, my_choice);
    }
    return final_points;
}
// {{{ points_p2
fn points_p2(rival: &str, mychoice: &str) -> i32
{
    let mut points = 0;
    match mychoice
    {
        "X" =>
        {
            if rival == "A"
            {
                points += 3;
            }
            else if rival == "B"
            {
                points += 1;
            }
            else
            {
                points += 2;
            }
        }
        "Y" =>
        {
            if rival == "A"
            {
                points += 4;
            }
            else if rival == "B"
            {
                points += 5;
            }
            else
            {
                points += 6;
            }
        }
        "Z" =>
        {
            if rival == "A"
            {
                points += 8;
            }
            else if rival == "B"
            {
                points += 9;
            }
            else
            {
                points += 7;
            }
        }
        _ =>
        {
            print!("WTF")
        }
    }
    return points;
}
// }}}

fn p2(content: &Vec<String>) -> i32
{
    let mut final_points = 0;
    for line in content
    {
        let fixedline = line.replace("\r", "");
        let rps_match: Vec<&str> = fixedline.split(' ').collect();
        let rival = rps_match[0];
        let my_choice = rps_match[1];
        final_points += points_p2(rival, my_choice);
    }
    return final_points;
}

fn main()
{
    let content = read_file_filter("input.txt".to_string());
    let p1_result = p1(&content);
    let p2_result = p2(&content);
    println!("P1: {}", p1_result);
    println!("P2: {}", p2_result);
}
