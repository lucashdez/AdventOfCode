import { readFileSync } from "node:fs";
/// {{{ P1
function P1(data: Array<string>, sample : any) {
    let result = {Id:  "-1", Coincidence: 0};
    for (const line of data) {
        // ID
        let ID_preproc = /Sue (?<ID>\d+)/.exec(line)?.groups?.ID;
        
        let id = null;
        if (ID_preproc !== undefined) {
            id = ID_preproc;
        }
        // Children
        const children_preproc = /children: (?<Children>\d+)/.exec(line)?.groups
            ?.Children;
        let children = null;
        if (children_preproc !== undefined) {
            children = children_preproc;
        }

        //Cats
        const cats_preproc = /cats: (?<Cats>\d+)/.exec(line)?.groups?.Cats;
        let cats = null
        if (cats_preproc !== undefined) {
            cats = cats_preproc;
        }

        //Samoyeds
        const samoyeds_preproc = /samoyeds: (?<Samoyeds>\d+)/.exec(line)?.groups?.Samoyeds;
        let samoyeds = null;
        if (samoyeds_preproc !== undefined) {
            samoyeds = samoyeds_preproc;
        }

        //Pomeranians
        const pomeranians_preproc = /pomeranians: (?<Pomeranians>\d+)/.exec(line)?.groups?.Pomeranians;
        let pomeranians = null;
        if (pomeranians_preproc !== undefined) {
            pomeranians = pomeranians_preproc;
        }

        //Akitas
        const akitas_preproc = /akitas: (?<Value>\d+)/.exec(line)?.groups?.Value;
        let akitas = null;
        if (akitas_preproc !== undefined) {
            akitas = akitas_preproc;
        }

        //Vizslas
        const vizslas_preproc = /vizslas: (?<Value>\d+)/.exec(line)?.groups?.Value;
        let vizslas = null;
        if (vizslas_preproc !== undefined) {
            vizslas = vizslas_preproc;
        }

        //Goldfish
        const goldfish_preproc = /goldfish: (?<Value>\d+)/.exec(line)?.groups?.Value;
        let goldfish = null;
        if (goldfish_preproc !== undefined) {
            goldfish = goldfish_preproc;
        }

        //Trees
        const trees_preproc = /trees: (?<Value>\d+)/.exec(line)?.groups?.Value;
        let trees = null;
        if (trees_preproc !== undefined) {
            trees = trees_preproc;
        }
        //Cars
        const cars_preproc = /cars: (?<Value>\d+)/.exec(line)?.groups?.Value;
        let cars = null;
        if (cars_preproc !== undefined) {
            cars = cars_preproc;
        }
        //Perfumes
        const perfumes_preproc = /perfumes: (?<Value>\d+)/.exec(line)?.groups?.Value;
        let perfumes = null;
        if (perfumes_preproc !== undefined) {
            perfumes = perfumes_preproc;
        }
        
        const sue = {ID: id,
                    Children: children,
                    Cats: cats,
                    Samoyeds: samoyeds,
                    Pomeranians: pomeranians,
                    Akitas: akitas,
                    Vizslas: vizslas,
                    Goldfish: goldfish,
                    Trees: trees,
                    Cars: cars,
                    Perfumes: perfumes}

        let coincidence = 0;
        if (sue.Children == sample.children) 
            coincidence++;
        if (sue.Cats == sample.cats )
            coincidence++;
        if (sue.Samoyeds == sample.samoyeds)
            coincidence++;
        if (sue.Pomeranians == sample.pomeranians)
            coincidence++;
        if (sue.Akitas == sample.akitas) 
            coincidence++;
        if (sue.Vizslas == sample.vizslas)
            coincidence++;
        if (sue.Goldfish == sample.goldfish)
            coincidence++;
        if (sue.Trees == sample.trees)
            coincidence++;
        if (sue.Cars == sample.cars)
            coincidence++;
        if (sue.Perfumes == sample.perfumes)
            coincidence++;

        if (coincidence > result.Coincidence) {
            result.Id = sue.ID || "-1";
            result.Coincidence = coincidence;
        }
        
    }
    return result.Id;
}
//}}}

function P2(data: Array<string>, sample: any) {
    let result = {Id:  "-1", Coincidence: 0};
    for (const line of data) {
        // ID
        let ID_preproc = /Sue (?<ID>\d+)/.exec(line)?.groups?.ID;
        
        let id = null;
        if (ID_preproc !== undefined) {
            id = ID_preproc;
        }
        // Children
        const children_preproc = /children: (?<Children>\d+)/.exec(line)?.groups
            ?.Children;
        let children = null;
        if (children_preproc !== undefined) {
            children = children_preproc;
        }

        //Cats
        const cats_preproc = /cats: (?<Cats>\d+)/.exec(line)?.groups?.Cats;
        let cats = null
        if (cats_preproc !== undefined) {
            cats = cats_preproc;
        }

        //Samoyeds
        const samoyeds_preproc = /samoyeds: (?<Samoyeds>\d+)/.exec(line)?.groups?.Samoyeds;
        let samoyeds = null;
        if (samoyeds_preproc !== undefined) {
            samoyeds = samoyeds_preproc;
        }

        //Pomeranians
        const pomeranians_preproc = /pomeranians: (?<Pomeranians>\d+)/.exec(line)?.groups?.Pomeranians;
        let pomeranians = null;
        if (pomeranians_preproc !== undefined) {
            pomeranians = pomeranians_preproc;
        }

        //Akitas
        const akitas_preproc = /akitas: (?<Value>\d+)/.exec(line)?.groups?.Value;
        let akitas = null;
        if (akitas_preproc !== undefined) {
            akitas = akitas_preproc;
        }

        //Vizslas
        const vizslas_preproc = /vizslas: (?<Value>\d+)/.exec(line)?.groups?.Value;
        let vizslas = null;
        if (vizslas_preproc !== undefined) {
            vizslas = vizslas_preproc;
        }

        //Goldfish
        const goldfish_preproc = /goldfish: (?<Value>\d+)/.exec(line)?.groups?.Value;
        let goldfish = null;
        if (goldfish_preproc !== undefined) {
            goldfish = goldfish_preproc;
        }

        //Trees
        const trees_preproc = /trees: (?<Value>\d+)/.exec(line)?.groups?.Value;
        let trees = null;
        if (trees_preproc !== undefined) {
            trees = trees_preproc;
        }
        //Cars
        const cars_preproc = /cars: (?<Value>\d+)/.exec(line)?.groups?.Value;
        let cars = null;
        if (cars_preproc !== undefined) {
            cars = cars_preproc;
        }
        //Perfumes
        const perfumes_preproc = /perfumes: (?<Value>\d+)/.exec(line)?.groups?.Value;
        let perfumes = null;
        if (perfumes_preproc !== undefined) {
            perfumes = perfumes_preproc;
        }
        
        const sue = {ID: id,
                    Children: children,
                    Cats: cats,
                    Samoyeds: samoyeds,
                    Pomeranians: pomeranians,
                    Akitas: akitas,
                    Vizslas: vizslas,
                    Goldfish: goldfish,
                    Trees: trees,
                    Cars: cars,
                    Perfumes: perfumes}

        let coincidence = 0;
        if (sue.Children == sample.children) 
            coincidence++;
        if (sue.Cats && sue.Cats > sample.cats )
            coincidence++;
        if (sue.Samoyeds == sample.samoyeds)
            coincidence++;
        if (sue.Pomeranians && sue.Pomeranians < sample.pomeranians)
            coincidence++;
        if (sue.Akitas == sample.akitas) 
            coincidence++;
        if (sue.Vizslas == sample.vizslas)
            coincidence++;
        if (sue.Goldfish && sue.Goldfish < sample.goldfish)
            coincidence++;
        if (sue.Trees && sue.Trees > sample.trees)
            coincidence++;
        if (sue.Cars == sample.cars)
            coincidence++;
        if (sue.Perfumes == sample.perfumes)
            coincidence++;

        if (coincidence > result.Coincidence) {
            result.Id = sue.ID || "-1";
            result.Coincidence = coincidence;
        }
        
    }
    return result.Id;
}

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
    const p2 = P2(dataArray, sampleSue);
    console.log(`P1: ${p1}\nP2: ${p2}`)
}

main();
