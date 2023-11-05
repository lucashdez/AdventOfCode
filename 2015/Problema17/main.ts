import  {powerSet} from "https://deno.land/x/combinatorics/mod.ts";

function powerSet(arr: readonly number[]) {
	return arr.reduce(
		(subsetsofar: number[][], elem:number) =>
			subsetsofar.concat(subsetsofar.map((subset) => subset.concat(elem))),
		[[]]
	)
}


function P1(sequence: Array<number>) {
	const p1_eq = 150;
	let seq = powerSet(sequence);
	let result = seq.filter((list) => {return list.reduce(function(a,b) { return a+b},0) == p1_eq})
	

	return result;
}

function P2(sequence) {
	let min_size = sequence.reduce((a,b) => {return (b.length < a)? b.length : a},999);
	let result = sequence.filter((f) => {return f.length == 4});
	return result
}




function main() {
	const containers = 	[33,14,18,20,45,35,16,35,1,13,18,13,50,44,48,6,24,41,30,42]
	let p1_res = P1(containers);
	let p2_res = P2(p1_res);
	console.log(`P1:${p1_res.length}\nP2:${p2_res.length}`);
}



main();
