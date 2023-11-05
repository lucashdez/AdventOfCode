async function draw(arr: boolean[][]) {
	let auxstr = "";
	for (let i = 0; i < arr.length; i++) {
		for (let j = 0; j < arr[i].length; j++) {
			auxstr += arr[i][j]? "#" : "."; 
		}
		auxstr += "\n";
	}

	//await new Promise((f) => setTimeout(f, 1000))
	console.clear();
	console.log(auxstr);
}

const switchOn()

function P1(arr: boolean[][]) {
	let iter = 4;
	let old_one = arr;
	let new_one = arr;

	for (let i = 0; i < iter; ++i) {
		for (let j = 0; j < arr[i].length; ++j) {

		}
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
	const data = readed.split("\n").map((line) => line.split("").map((c) => c === "#"));

	draw(data);
	let p1_res = P1(data);
	let p2_res = P2();
	console.log(`P1: ${p1_res}\nP2: ${p2_res}`);
}

main();
