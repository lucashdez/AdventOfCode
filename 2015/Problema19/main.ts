class Rule {
	public f = "";
	public r = "";
	public constructor(from, to) {
		this.f = from;
		this.r = to;
	}
}

async function draw(str) {
	await new Promise(r => setTimeout(r, 100));
	console.clear();
	console.log(str);
}

function replace(str, f, r, pos) {
	if (pos > 0) 
		return str.slice(0, pos) + r + str.slice(pos + f.length);
	else 
		return r + str.slice(pos + f.length);
}

async function P2(rules, readonly str) {
	let adn_set = new Set();
	let variablestr = str.slice();
	let count = 0;
	while(variablestr !== "e") {
		rules.map(rule => {
			let rg = new RegExp(rule.r, "g");
			let result;
			while ((result = rg.exec(variablestr))) {
				let ncom = replace(variablestr, rule.r, rule.f, result.index);
				variablestr = ncom;
				count += 1
				console.log(variablestr)
			}
		})
	}
	return count;
}

function P1 (rules, readonly str) {
	let adn_set = new Set();
	rules.map(rule => {
		let rg = new RegExp(rule.f, "g");
		let result;
		while ((result = rg.exec(str))) {
			let ncom = replace(str, rule.f, rule.r, result.index);
			adn_set.add(ncom);
		}
	})
	return adn_set.size;
}

async function main() {
	let decoder = new TextDecoder("utf-8")
	let data = decoder.decode(Deno.readFileSync("input.txt", "utf-8")).split("\n").map(s => s.replace("\r", "")).filter(d => d != "");
	let adn = data[data.length-1] 
	let transitions = []
	for (let i = 0; i < data.length-1; ++i) {
		transitions.push(data[i]);
	}
	const rules = transitions.map(t => {
		let s = t.split("=>").map(si => si.replace(" ", ""));
		return new Rule(s[0], s[1])
	});

	let p1 = await P1(rules, adn);
	let p2 = await P2(rules, adn);
	console.log(`P1: ${p1}\nP2: ${p2}`);
}

main();
