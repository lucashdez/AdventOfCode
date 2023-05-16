import * as fs from "node:fs";

const data = fs
    .readFileSync("input.txt", "utf-8")
    .split("\n")
    .filter((d) => d !== "");
//Alice would gain 54 happiness units by sitting next to Bob.
//Alice would lose 79 happiness units by sitting next to Carol.
const re =
    /(?<NAME>[a-zA-Z]+) would (?<ACTION>(gain|lose)) (?<VALUE>\d+) happiness units by sitting next to (?<CONDITION>[a-zA-Z]+)./;
let names: Set<string> = new Set();
let relations = Object.create(null);

data.forEach((s) => {
    let result = re.exec(s).groups;
    names.add(result.NAME);
    relations[result.NAME] = relations[result.NAME] || {};
    relations[result.NAME][result.CONDITION] =
        result.ACTION === "gain"
            ? parseInt(result.VALUE)
            : -parseInt(result.VALUE);
});
let aNames = [];
names.forEach((n) => aNames.push(n));
//{{{ permute
const permute = (arr = []) => {
    let res = [];
    const helper = (arr2: Array<string>) => {
        if (arr2.length == arr.length) return res.push(arr2);
        for (let e of arr) {
            if (!arr2.includes(e)) helper([...arr2, e]);
        }
    };
    helper([]);
    return res;
};
//}}}
let p = permute(aNames);
let nres = [];
p.forEach((aNames) => {
    let nrel = 0;
    for (let i = 0; i < aNames.length; ++i) {
        if (i === aNames.length - 1) {
            nrel +=
                relations[aNames[i]][aNames[0]] +
                relations[aNames[0]][aNames[i]];
        } else {
            nrel +=
                relations[aNames[i]][aNames[i + 1]] +
                relations[aNames[i + 1]][aNames[i]];
        }
    }
    nres.push(nrel);
});
//{{{ min
const min = (arr) => {
    let min = 9999999999999;
    for (let n of arr) {
        if (n < min) min = n;
    }
    return min;
};
//}}}
//{{{max
const max = (arr) => {
    let max = -99999999999;
    for (let n of arr) {
        if (n > max) max = n;
    }
    return max;
};
//}}}

console.log(`
P1: ${max(nres)};
`);
