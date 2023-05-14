import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf-8");

let counter = 0;
let times = 0;
let floor = 0;

for (let c of data) {
  if (c === "(") {
    counter++
    floor++;
  };
  if (c === ")") {
    counter--;
    floor--;
  } 
  times++;
  //if (counter === -1) break;
}

console.log(counter);
console.log(times);
console.log("Floor: " + floor);
