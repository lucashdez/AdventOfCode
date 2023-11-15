async function P1(readonly input) {
	const presents = []
	
	for (let e = 1; e < input / 10; e++) {
		let visits = 0
		for (let i = e; i < input / 10; i = i + e) {
			if (!presents[i]) presents[i] = 10
			presents[i] = presents[i] + e * 10;
		}
	}
	return presents.reduce((min, current, index) => (min === 0 && current >= input) ? min = index : min, 0);
}

async function P2(input) {
	const presents2 = []
	for (let e = 1; e < input / 10; e++) {
		let visits = 0
		for (let i = e; i < input / 10; i = i + e) {
			if (visits < 50) {
				if (!presents2[i]) presents2[i] = 11
				presents2[i] = presents2[i] + e * 11
				visits = visits + 1
			}
		}
	}
	return presents2.reduce((min, current, index) => (min === 0 && current >= input) ? min = index : min, 0)
}

async function main() {
	const input = 36000000

	let ap1 = await P1(input)
	let ap2 = await P2(input)

	console.log(`P1: ${ap1}\nP2: ${ap2}`);

}

main();
