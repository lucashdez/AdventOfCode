const size = 6
async function draw(readonly arr: boolean[][]) {
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

const nei = [{x:-1, y:-1},
			 {x:-1, y:0},
			 {x:-1, y:1},
			 {x:0, y:-1},
			 {x:0, y:1},
			 {x:1, y:-1},
			 {x:1, y:0},
			 {x:1, y:1}]

function neighboors(readonly grid: boolean[][], i: number, j: number) {
	let num = 0;
	nei.map((d) => {
		if(valid(i+d.x, j+d.y)) {
			num += grid[i+d.x][j+d.y]? 1 : 0;
		}
	});
	return num;
}

async function P1(arr: boolean[][]) {
	let iter = 100;
	let old_one = arr;
	let new_one = arr;

	for (let iterator = 0; iterator < iter; iterator++) {
		for(let i = 0; i < size; i++) {
			for(let j = 0; j < size; j++) {
				const nn = neighboors(old_one, i, j);
				if (old_one[i][j]) {
					new_one[i][j] = (nn != 2 && nn != 3)? false : true;
				} else {
					new_one[i][j] = (nn === 3)? true : false;
				}
			}
		}
		old_one = new_one;
		await draw(old_one)
	}

	let count = 0;
	new_one.forEach((row) => row.forEach((col) => count += col ? 1 : 0));
	return count;
}

function P2() {
}



async function main() {
	const readed = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..\n";
	const data = readed.split("\n").map((line) => line.split("").map((c) => c === "#")).filter(d => d.length !== 0);

	let p1_res =  P1(data);
	let p2_res =  P2();
	console.log(`P1: ${p1_res}\nP2: ${p2_res}`);
}

main();
