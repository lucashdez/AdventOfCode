import {readFileSync} from 'fs';
const log = console.log;
const data = readFileSync('input.tst', 'utf-8')
//{{{ House
class House {
  x = null;
  y = null;
  visited = false;
  nPresents = 0;
  constructor(x:number, y:number) {
    this.x = x;
    this.y = y;
    this.nPresents = 0;
    this.visited = false;
  }
  visit() {
    if (!this.visited) {
      this.visited = true
    }
    this.nPresents++;
  }
};
///}}}
//{{{ List
class List {
  arr = [];
  newHouses = 0;
  constructor() {
    this.arr = [];
  }
  #search(x:number, y:number) {
    for(let i = 0; i < this.arr.length; ++i) {
      if (this.arr[i].x === x && this.arr[i].y === y) {
        return i;
      }
    }
    return null;
  }
  visit_house(x:number, y:number) {
    if(this.#search(x,y) === null) {
      this.arr.push(new House(x,y))
      this.newHouses++;
    }
    let index:number = this.#search(x,y);
    this.arr[index].visit();

  }
}
//}}}
function main() {
  let sX = 0;
  let sY = 0;
  let rX = 0;
  let rY = 0;
  let tMap = new List;
  tMap.visit_house(sX, sY);
  tMap.visit_house(rX, rY);
  let counter = 1;
  for (const c of data) {
    if (counter % 2 === 0) {
      switch(c) {
        case '^':
          rY++;
          tMap.visit_house(rX, rY);
          break;
        case '>':
          rX++;
          tMap.visit_house(rX, rY);
          break;
        case 'v':
          rY--;
          tMap.visit_house(rX,rY);
          break;
        case '<':
          rX--;
          tMap.visit_house(rX, rY);
          break;
        default:
          log(`WTF is ${c}`)
      }
    } else {
      switch(c) {
        case '^':
          sY++;
          tMap.visit_house(sX, sY);
          break;
        case '>':
          sX++;
          tMap.visit_house(sX, sY);
          break;
        case 'v':
          sY--;
          tMap.visit_house(sX,sY);
          break;
        case '<':
          sX--;
          tMap.visit_house(sX,sY);
          break;
        default:
          log(`WTF is ${c}`)
      }
    }
    counter++;
  }
  
  log(`Result: ${tMap.newHouses}`)

}

main();
