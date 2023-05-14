import { readFileSync } from "node:fs";
const data = readFileSync("./input.txt", "utf-8");
const separatedOps = data.split("\n");

let bow = 0;
let ribbon = 0;
let squareFeet = 0;
for (let i = 0; i < separatedOps.length - 1; ++i) {
    const sides = separatedOps[i].split("x");
    const nArr = [Number(sides[0]), Number(sides[1]), Number(sides[2])];
    const lenght = nArr[0];
    const width = nArr[1];
    const height = nArr[2];
    const sortedSurfaces = [
        lenght * width,
        width * height,
        height * lenght,
    ].sort((a, b) => a - b);

    const boxSize =
        sortedSurfaces[0] +
        2 * (sortedSurfaces[0] + sortedSurfaces[1] + sortedSurfaces[2]);

    squareFeet += boxSize;

    const s = nArr.sort((a, b) => a - b);
    bow += 2 * s[0] + 2 * s[1];
    ribbon += s[0] * s[1] * s[2];
}
console.log(`
Part1:\n 
    SquareFeet: ${squareFeet}
            
Part2:\n
    bow: ${bow}
    ribbon: ${ribbon}
    sum: ${bow + ribbon}
`);
