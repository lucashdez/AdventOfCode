async function draw(readonly arr: number[][]) {
	let auxstr = "";
	for (let i = 0; i < arr.length; i++) {
		for (let j = 0; j < arr[i].length; j++) {
			auxstr += arr[i][j]? "█" : "x"; 
		}
		
		auxstr += "\n";
	}
	await new Promise(r => setTimeout(r, 100));
	console.clear()
	console.log(auxstr);
}

function get_neighboors(arr: number[][], row, col) {
	let count = 0;
	for (let i = row-1; i <= row+1; i++) {
		for(let j = col-1; j <= col+1; j++) {
			if(arr[i][j] == 1)
				count += 1;
		}
	}
	
	return count;
}

function update(original: number[][], internal_size: number) {
	let newa = original;
	for(let i = 1; i < internal_size + 1; i++) {
		for(let j = 1; j < internal_size + 1; j++) {
			const nn = get_neighboors(original, i, j);
			let state: number;
			if (original[i][j] == 1) 
				state = (nn == 3 || nn == 4)? 1 : 0;
			else 
				state = (nn == 3)? 1 : 0;
			newa[i][j] = state;
		}
	}
	original = newa;
}

async function P1(arr: number[][], iter: number, internal_size: number) {
	for (let iterator = 0; iterator < iter; iterator++) {
		update(arr, internal_size);
		await draw(arr);
	}
	let result = arr
	return result
}

function P2() {
	return []
}


function count(arr: number[][]) {
	let count = 0;
	for (let i = 0; i < arr.length; i++)
		for (let j = 0; j < arr[i].length; j++)
			count += arr[i][j]? 1 : 0;
	return count;
}


async function main() {
	//fixed data
	const internal_size = 100;
	const iterations = 100;

	const decoder = new TextDecoder("utf-8");
	const readed = decoder.decode(Deno.readFileSync("input.txt"));
	const data = readed.split("\n").map((line) => line.split("").map((c) => (c === "#")? 1 : 0).filter(d => d.length !== 0));

	let windowed_arr: number[][] = [];
	for(let i = 0; i < internal_size + 2; i++) {
		let void_arr = []
		for (let j = 0; j < internal_size + 2; ++j)
			void_arr.push(0);
		windowed_arr.push(void_arr);
	}
	for(let i = 0; i < internal_size; ++i) 
		for(let j = 0; j < internal_size; ++j)
			windowed_arr[i+1][j+1] = data[i][j];

	await(draw(windowed_arr))
	let ap1 = await P1(windowed_arr, iterations, internal_size)
	let ap2 = P2();

	let p1_res = count(ap1);
	let p2_res = count(ap2);
	console.log(`P1: ${p1_res}\nP2: ${p2_res}`);
}

main();
