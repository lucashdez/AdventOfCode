function main() {
	let decoder = new TextDecoder("utf-8")
	let data = decoder.decode(Deno.readFileSync("test.txt", "utf-8")).split("\n").map(s => s.replace("\r", "")).filter(d => d != "");
	let adn = data[data.length-1] 
	let transitions = []
	for (let i = 0; i < data.length-1; ++i) {
		transitions.push(data[i]);
	}
	console.log(adn);
	console.log(transitions);
	const rules = transitions.map(t => {
		let s = t.split("=>").map(si => si.replace(" ", ""));
		let obj = new Object();
		obj[s[0]] = s[1];
		return obj;
	});

	
}

main();
