import { readFileSync } from "node:fs";
const data = readFileSync("./input.txt", "utf-8");
const dataArr: string[] = data.split("\n");
const findRepeated = (s: string) => {
    let solution = null;
    for (let i = 0; i < s.length - 1; ++i) {
        const newReg = s.substring(i, i + 2);
        const matches = s.match(RegExp(newReg, "g"));
        if (matches != null && matches.length >= 2) {
            solution = matches;
        }
    }
    return solution !== null;
};

const aoa = (s: string) => {
    for (let i = 0; i < s.length; ++i) {
        if (s.charAt(i) === s.charAt(i + 2)) return true;
    }
    return false;
};

function main() {
    let counter = 0;
    for (const string of dataArr) {
        if (findRepeated(string) && aoa(string)) counter++;
    }
    console.log(counter);
}

main();
