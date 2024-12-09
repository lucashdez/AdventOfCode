fn check_n(v: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
	if (i as isize - 3) >= 0 {
		if v[i][j] == 'X'
			&& v[i-1][j] == 'M'
			&& v[i-2][j] == 'A'
			&& v[i-3][j] == 'S'
		{
			return 1
		}
	}
	0
}

fn check_nw(v: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
	if (i as isize - 3) >= 0 && (j as isize - 3 >= 0) {
		if v[i][j] == 'X'
			&& v[i-1][j-1] == 'M'
			&& v[i-2][j-2] == 'A'
			&& v[i-3][j-3] == 'S'
		{
			return 1
		}
	}
	0
}


fn check_w(v: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
	if (j as isize - 3) >= 0 {
		if v[i][j] == 'X'
			&& v[i][j - 1] == 'M'
			&& v[i][j - 2] == 'A'
			&& v[i][j - 3] == 'S'
		{
			return 1
		}
	}
	0
}

fn check_sw(v: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
	if (j as isize - 3) >= 0  && (i + 3) < v.len() {
		if v[i][j] == 'X'
			&& v[i + 1][j - 1] == 'M'
			&& v[i + 2][j - 2] == 'A'
			&& v[i + 3][j - 3] == 'S'
		{
			return 1
		}
	}
	0
}

fn check_s(v: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
	if (i + 3) < v.len() {
		if v[i][j] == 'X'
			&& v[i+1][j] == 'M'
			&& v[i+2][j] == 'A'
			&& v[i+3][j] == 'S'
		{
			return 1
		}
	}
	0
}

fn check_se(v: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
	if (i + 3) < v.len() && i < v.len() && (j + 3) < v[0].len() {
		if v[i][j] == 'X'
			&& v[i+1][j+1] == 'M'
			&& v[i+2][j+2] == 'A'
			&& v[i+3][j+3] == 'S'
		{
			return 1
		}
	}
	0
}


fn check_e(v: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
	if (j + 3) < v[i].len()  {
		if v[i][j] == 'X'
			&& v[i][j + 1] == 'M'
			&& v[i][j + 2] == 'A'
			&& v[i][j + 3] == 'S'
		{
			return 1
		}
	}
	0
}

fn check_ne(v: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
	if (j + 3) < v[i].len() && (i as isize - 3) >= 0  {
		if v[i][j] == 'X'
			&& v[i - 1][j + 1] == 'M'
			&& v[i - 2][j + 2] == 'A'
			&& v[i - 3][j + 3] == 'S'
		{
			return 1
		}
	}
	0
}


fn check_sides(v: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
	let mut sum = 0;
	sum += check_n(v, i, j) as i64;
	sum += check_nw(v, i, j) as i64;
	sum += check_w(v, i, j) as i64;
	sum += check_sw(v, i, j) as i64;
	sum += check_s(v, i, j) as i64;
	sum += check_se(v, i, j) as i64;
	sum += check_e(v, i, j) as i64;
	sum += check_ne(v, i, j) as i64;
	sum
}

fn check_x(v: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
	let mut sum = 0;
	if (i as isize - 1) < 0 {return 0;} 
	if (j as isize - 1) < 0 {return 0;}
	if (i + 1) >= v.len() {return 0;}
	if (j + 1) >= v[0].len() {return 0;}

	let mut count = 0;
	if v[i-1][j-1] == 'M' && v[i+1][j+1] == 'S' {count += 1;}
	if v[i-1][j+1] == 'M' && v[i+1][j-1] == 'S' {count += 1;}
	if v[i+1][j-1] == 'M' && v[i-1][j+1] == 'S' {count += 1;}
	if v[i+1][j+1] == 'M' && v[i-1][j-1] == 'S' {count += 1;}

	sum += count/2;

	sum
}


fn main() {

	let read = String::from("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
");
	let read = std::fs::read_to_string("input/input.txt").expect("cant open");

	let v: Vec<Vec<char>> = read.replace("\r", "")
		.split("\n").filter(|d| *d!="")
		.map(|x| x.chars().collect::<Vec<char>>()).collect();


	let mut tot_p1 = 0;
	for r in 0..v.len() {
		for c in 0..v[r].len() {
			if v[r][c] == 'X' {
				tot_p1 += check_sides(&v, r, c);
			}
		}
	}

	let mut tot_p2 = 0;
	for r in 0..v.len() {
		for c in 0..v[r].len() {
			if v[r][c] == 'A' {
				tot_p2 += check_x(&v, r, c);
			}
		}
	}

	println!("{tot_p1}\n{tot_p2}");
}


