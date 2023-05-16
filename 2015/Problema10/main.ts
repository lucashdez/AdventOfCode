import * as fs from "node:fs";
const data = fs.readFileSync("input.txt", "utf-8");

let input = data;

console.log(input);

const process = (s: string) => {
    let newString = "";
    let chars = s.match(/(\w)\1*/g);
    chars.forEach((char) => {
        newString += char.length + "" + char[0];
    });
    return newString;
};

// LOOP
for (let i = 0; i < 50; ++i) {
    input = process(input);
}

console.log(input.length);
