import { readFileSync } from "node:fs";

function P1(data: Array<string>, sample: Object) {
    let result = {};
    for (const line of data) {
        // ID
        let ID_preproc = /Sue (?<ID>\d+)/.exec(line)?.groups.ID;
        let ID = null;
        if (ID_preproc !== undefined) {
            ID = ID_preproc;
        }
        // Children
        let children_preproc = /children: (?<Children>\d+)/.exec(line)?.groups
            .Children;
        let children = null;
        if (children_preproc !== undefined) {
            children = children_preproc;
        }

        //Cats
        let cat_preproc = /cats: /;

        //Samoyeds
        //Pomeranians
        //Akitas
        //Vizslas
        //Goldfish
        //Trees
        //Cars
        //Perfumes

        console.log(`ID: ${ID} 
                    Children: ${children}`);

        result = line;
    }
    return data;
}

function P2(data: Array<string>) {}

function main() {
    const sampleSue = {
        children: 3,
        cats: 7,
        samoyeds: 2,
        pomeranians: 3,
        akitas: 0,
        vizslas: 0,
        goldfish: 5,
        trees: 3,
        cars: 2,
        perfumes: 1,
    };
    const data = readFileSync("input.txt", "utf-8");
    const dataArray = data.split("\n").filter((line: string) => line !== "");
    const p1 = P1(dataArray, sampleSue);
}

main();
