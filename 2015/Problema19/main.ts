class Rule {
	public f = "";
	public r = "";
	public constructor(from, to) {
		this.f = from;
		this.r = to;
	}
}

function replace(str, w, pos) {
	if (pos > 0) 
		return str.slice(0, pos) + w + str.slice(pos + 1);
	else 
		return w + str.slice(pos + 1);
}

function P2(rules, readonly str) {
	let result = 0;

	return result;
}

function P1 (rules, readonly str) {
	let adn_set = new Set();
	rules.map(rule => {
		let srep = rule.f.length
		for (let i = 0; i < str.length; ++i) {
			switch (srep) {
				case 1:
					if (str[i] == rule.f) {
						let ncom = replace(str, rule.r, i);
						adn_set.add(ncom);
					}
					break;
				case 2:
					if (i < str.length-1) {
						if(str[i] == rule.f[0] && str[i+1] == rule.f[1]) {
							console.log(`Match ${rule.f}, in ${i}`)
							let newstr = str.slice(0,i+1) + str.slice(i+2);
							if (rule.f == "Ca" && i == 3)  console.log(newstr);
							let ncom = replace(newstr, rule.r, i);
							if (rule.f == "Ca" && i == 3)  console.log(ncom);
							adn_set.add(ncom);
						}
					}
					break;
				default:
					console.log("TF")
			}
			
		}
	})
	return adn_set.size;
}

function main() {
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

	let p1 = P1(rules, adn);
	let p2 = P2(rules, adn);
	console.log(`P1: ${p1}\nP2: ${p2}`);
}

main();
