const INPUTDATA = 2;
async function draw(readonly arr: number[][]) {
	let auxstr = "";
	for (let i = 0; i < arr.length; i++) {
		for (let j = 0; j < arr[i].length; j++) {
			auxstr += arr[i][j]? "█" : "x"; 
		}
		auxstr += "\n";
	}
	await new Promise(resolve => setTimeout(resolve, 50)); 
	console.clear();
	console.log(auxstr);
}

function get_neighboors(arr: number[][], row, col) {
	let count = 0;
	for (let i = row-1; i <= row+1; i++) {
		for(let j = col-1; j <= col+1; j++) {
			if(arr[i][j] === 1)
				count += 1;
		}
	}
	
	return count;
}

function copy_array(readonly from: number[][], dest: number[][]) {
	for (let i = 0; i < from.length; ++i)
		for (let j = 0; j < from[i].length; ++j)
			dest[i][j] = from[i][j];
}

function generate_arr(m:number,n:number) {
	let newarr = []
	for (let i = 0; i < m; ++i) {
		let auxarr = []
		for(let j = 0; j < n; ++j)
			auxarr.push(0);
		newarr.push(auxarr);
	}
	return newarr;
}

function update(original: number[][], internal_size: number, stuck: boolean = false) {
	let newa: number[][] = generate_arr(original.length, original[0].length) ;
	copy_array(original, newa);

	for(let i = 1; i < internal_size + 1; i++) {
		for(let j = 1; j < internal_size + 1; j++) {
			const nn = get_neighboors(original, i, j);
			let state: number;
			if (original[i][j] === 1) {
				state = (nn === 3 || nn === 4)? 1 : 0;
			} 
			else {
				state = (nn === 3)? 1 : 0;
			}
			newa[i][j] = state;
			if (stuck) {
				newa[1][1] = 1;
				newa[internal_size][internal_size] = 1
				newa[1][internal_size] = 1
				newa[internal_size][1] = 1
				
			}
		}
	}
	copy_array(newa, original)
}

async function P1(arr: number[][], iter: number, internal_size: number) {
	for (let iterator = 0; iterator < iter; iterator++) {
		update(arr, internal_size);
		await draw(arr);
	}
	let result = arr
	return result
}

async function P2(arr: number[][], iter: number, internal_size: number) {
	for (let iterator = 0; iterator < iter; iterator++) {
		update(arr, internal_size, true);
				await draw(arr);
	}
	let result = arr
	return result
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

	let windowed_arr_p1 = generate_arr(windowed_arr.length, windowed_arr[0].length);
	copy_array(windowed_arr, windowed_arr_p1);
	windowed_arr[1][1] = 1;
	windowed_arr[internal_size][internal_size] = 1
	windowed_arr[1][internal_size] = 1
	windowed_arr[internal_size][1] = 1


	draw(windowed_arr);
	let ap1 = await P1(windowed_arr_p1, iterations, internal_size);
	let ap2 = await P2(windowed_arr, iterations, internal_size);


	let p1_res = count(ap1);
	let p2_res = count(ap2);
	console.log(`P1: ${p1_res}\nP2: ${p2_res}`);
}

main();
