import { readFileSync } from "node:fs";
//{{{ Lights Init
let lArr = [];
for (let i = 0; i < 1000; ++i) {
    let lights = [];
    for (let j = 0; j < 1000; ++j) {
        lights.push(0);
    }
    lArr.push(lights);
}
//}}}

const data = readFileSync("./input.txt", "utf-8");

let dataArr = data.split("\n");
dataArr = dataArr.filter((d: string) => d !== "");
for (let order of dataArr) {
    let oArr = order.split(" ");
    let i1: number;
    let i2: number;
    let f1: number;
    let f2: number;
    let command: string;
    if (oArr.length === 4) {
        let ini = oArr[1].split(",");
        i1 = Number(ini[0]);
        i2 = Number(ini[1]);
        let fin = oArr[3].split(",");
        f1 = Number(fin[0]);
        f2 = Number(fin[1]);
        command = "toggle";
    } else {
        command = oArr[1];
        let ini = oArr[2].split(",");
        i1 = Number(ini[0]);
        i2 = Number(ini[1]);
        let fin = oArr[4].split(",");
        f1 = Number(fin[0]);
        f2 = Number(fin[1]);
    }
    for (let i = i1; i <= f1; ++i) {
        for (let j = i2; j <= f2; ++j) {
            switch (command) {
                // UNCOMMENT FOR PART 1
                case "toggle":
                    lArr[i][j] += 2;
                    //if (lArr[i][j] === 1) lArr[i][j] = 0;
                    //else lArr[i][j] = 1;
                    break;
                case "on":
                    lArr[i][j] += 1;
                    break;
                case "off":
                    lArr[i][j] -= lArr[i][j] > 0 ? 1 : 0;
                    break;
                default:
                    console.log("WTFWTF");
            }
        }
    }
}

let counter = 0;
for (let i = 0; i < 1000; ++i)
    for (let j = 0; j < 1000; ++j) counter += lArr[i][j];
console.log(counter);
