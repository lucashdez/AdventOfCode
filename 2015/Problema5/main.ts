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

const count_different_vowels = (s: string) => {
    let solution = 0;
    for (let i = 0; i < s.length; ++i) {
        if (
            s.charAt(i) === "a" ||
            s.charAt(i) === "e" ||
            s.charAt(i) === "i" ||
            s.charAt(i) === "o" ||
            s.charAt(i) === "u"
        ) {
            solution++;
        }
    }
    return solution >= 3;
};

const twoinarow = (s: string) => {
    for (let i = 0; i < s.length - 1; ++i) {
        if (s.charAt(i) === s.charAt(i + 1)) return true;
    }
    return false;
};

const forbidden = (s: string) => {
    return (
        s.includes("ab") ||
        s.includes("cd") ||
        s.includes("pq") ||
        s.includes("xy")
    );
};

function main() {
    let counter_p2 = 0;
    let counter_p1 = 0;

    for (const string of dataArr) {
        if (
            count_different_vowels(string) &&
            twoinarow(string) &&
            !forbidden(string)
        )
            counter_p1++;
    }
    for (const string of dataArr) {
        if (findRepeated(string) && aoa(string)) counter_p2++;
    }
    console.log(`
                P1: ${counter_p1},
                P2: ${counter_p2}
                `);
}

main();
