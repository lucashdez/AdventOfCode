const size = 6
async function draw(readonly arr: boolean[][]) {
	let auxstr = "";
	for (let i = 0; i < size; i++) {
		for (let j = 0; j < size; j++) {
			auxstr += arr[i][j]? "#" : "."; 
		}
		auxstr += "\n";
	}

	//await new Promise((f) => setTimeout(f, 1000))
	console.clear();
	console.log(auxstr);
}


function neighboors(readonly grid: boolean[][], row: number, col: number) {
	let num = 0;
	if( row>0 ) {
		if( col > 0 && grid[row-1][col-1]) num += 1;
		if( grid[row-1][col]) num += 1;
		if( col < size-1 && grid[row-1][col+1]) num += 1;
	}
    if( col > 0 && grid[row][col-1]) num += 1;
    if( col < size-1 && grid[row][col+1]) num += 1;
	if( row< size-1 ) {
		if( col > 0 && grid[row+1][col-1]) num += 1;
		if( grid[row+1][col]) num += 1;
		if( col < size-1 && grid[row+1][col+1]) num += 1;
	}
	
	return num;
}

function P1(arr: boolean[][]) {
	let iter = 4;
	let old_one = arr;
	let new_one = arr;

	for (let iterator = 0; iterator < iter; iterator++) {
		for(let i = 0; i < size; i++) {
			for (let j = 0; j < size; j++) {
				let nn = neighboors(old_one, i, j);
				if (old_one[i][j]) {
					new_one[i][j] = (nn==2 || nn == 3);
				} else {
					new_one[i][j] = (nn == 3);
				}
			}
		}
		old_one = new_one;
		draw(new_one);
	}
}

function P2() {
}



function main() {
	const readed = `
.#.#.#
...##.
#....#
..#...
#.#..#
####..
`;
	//const data = readed.split("\n").map((line) => line.split("").map((c) => c === "#"));
	const data = [[false,true,false,true,false,true],
				  [false,false,false,true,true,false],
				  [true,false,false,false,false,true],
				  [false,false,true,false,false,false],
				  [true,false,true,false,false,true],
				  [true,true,true,true,false,false]] 
	draw(data);

	let p1_res = P1(data);
	let p2_res = P2();
	console.log(`P1: ${p1_res}\nP2: ${p2_res}`);
}

main();
