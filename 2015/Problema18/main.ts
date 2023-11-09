const size = 6
async function draw(readonly arr: number[][]) {
	let auxstr = "";
	for (let i = 0; i < size; i++) {
		for (let j = 0; j < size; j++) {
			auxstr += arr[i][j]? "█" : " "; 
		}
		
		auxstr += "\n";
	}
	await new Promise(r => setTimeout(r, 100));
	console.clear()
	console.log(auxstr);
}

function valid(i,j) {
	return i>=0 && i<size && j>=0 && j<size
}

function get_data(readonly grid:number[][], i, j) {
	if(valid(i, j))
		return grid[i][j]
	return 0
}

function get_sum(readonly grid:number[][], i, j) {
	let sum = 0;

	sum += get_data(grid, i - 1, j - 1) ;
	sum += get_data(grid, i - 1, j) ;
	sum += get_data(grid, i - 1, j + 1) ;
	sum += get_data(grid, i, j - 1) ;
	sum += get_data(grid, i, j + 1) ;
	sum += get_data(grid, i + 1, j - 1) ;
	sum += get_data(grid, i + 1, j) ;
	sum += get_data(grid, i + 1, j + 1) ;

	return sum
}

function update(original: number[][]) {
	let newa = original;
	for(let i = 0; i < size; i++) {
		for(let j = 0; j < size; j++) {
			const nn = get_sum(original, i, j);
			let state: number;
			if (original[i][j] == 1) 
				state = (nn == 2 || nn == 3)? 1 : 0;
			else 
				state = (nn == 3)? 1 : 0;
			newa[i][j] = state;
		}
	}
	original = newa;
}

async function P1(arr: number[][]) {
	let iter = 4;
	for (let iterator = 0; iterator < iter; iterator++) {
		update(arr);
		await draw(arr);
	}
	let result = arr
	return arr;
}

function P2() {
}


function count(arr: number[][]) {
	let count = 0;
	for (let i = 0; i < size; i++)
		for (let j = 0; j < size; j++)
			count += arr[i][j]? 1 : 0;
	return count;
}


async function main() {
	const readed = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..\n";
	const data = readed.split("\n").map((line) => line.split("").map((c) => (c === "#")? 1 : 0).filter(d => d.length !== 0));

	let ap1 = await P1(data)
	let p1_res =  count(ap1);
	
	let p2_res =  P2();
	console.log(`P1: ${p1_res}\nP2: ${p2_res}`);
}

main();
