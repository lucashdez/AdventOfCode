async function draw() {
}

async function P1() {
}

async function P2() {
}

async function main() {

	let ap1 = await P1()
	let ap2 = await P2()

	console.log(`P1: ${ap1}\nP2: ${ap2}`);

}

main();
