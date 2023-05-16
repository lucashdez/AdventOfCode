import * as fs from "node:fs";

const input = fs.readFileSync("input.txt", "utf-8");

let data = input.split("\n");
data = data.map((s: string) => s.replace("\r", ""));
data = data.filter((d: string) => d !== "");

let p1 = 0;
let p2 = 0;
data.forEach((s: string) => {
    p1 += s.length - eval(s).length;
    p2 += JSON.stringify(s).length - s.length;
});

console.log(`
Part 1: ${p1}
Part 2: ${p2}
            `);
