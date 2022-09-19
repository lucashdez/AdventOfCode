import * as fs from 'fs';
let data = fs.readFileSync('input.txt', 'utf-8').split('\n').filter(d=>d!=='');
const re = /(?<NAME>[a-zA-Z]+) can fly (?<SPEED>\d+) km\/s for (?<LIMIT>\d+) seconds, but then must rest for (?<REST>\d+) seconds./;
let flyers = Object.create(null);
//{{{ INIT
data.forEach(s => {
  let result = re.exec(s).groups;
  flyers[result.NAME] = {
    distance: 0,
    wait: 0,
    speed: parseInt(result.SPEED),
    limit: parseInt(result.LIMIT),
    left: parseInt(result.LIMIT),
    rest: parseInt(result.REST),
    points: 0,
  };
});

//}}}
//{{{ MAX
const max = (arr:Array<number>) => {
  let max = 0;
  arr.forEach(e => {
    if (max < e) max = e;
  })
  return max;
}
//}}}
let keys = Object.keys(flyers);
//{{{ p1(flyers, keys)
const p1 = (flyers, keys) => {
  for(let n = 0; n < 2503; n++) {
    keys.forEach(key => {
      if (flyers[key].wait === 0) {
        flyers[key].distance += flyers[key].speed;
        flyers[key].left--;
        if(flyers[key].left === 0) {
          flyers[key].wait = flyers[key].rest;
          flyers[key].left = flyers[key].limit;
        }
      } else {
        flyers[key].wait--;
      }
    });
  }
  let distances = [];
  keys.forEach(key => {
    distances.push(flyers[key].distance);
    flyers[key].distance = 0;
    flyers[key].wait = 0;
    flyers[key].left = flyers[key].limit;
  });
  return distances
}
//}}}
//{{{ P2
const p2 = (flyers, keys) => {
  for (let n = 0; n < 2503; ++n) {
    let c_max = {value: 0, key: ""};
    keys.forEach(key => {
      if (flyers[key].wait === 0) {
        flyers[key].distance += flyers[key].speed;
        flyers[key].left--;
        if(flyers[key].left === 0) {
          flyers[key].wait = flyers[key].rest;
          flyers[key].left = flyers[key].limit
        }
      } else {
        flyers[key].wait--;
      }
      if(c_max.value < flyers[key].distance) {
        c_max.key = key;
        c_max.value = flyers[key].distance;
      }
    });
    keys.forEach(key => {
      if(c_max.value === flyers[key].distance) {
        flyers[key].points++;
      }
    })
    c_max.key = "";
    c_max.value = 0;
  }
  let points = [];
  keys.forEach(key => {
    points.push(flyers[key].points);
  })
  return points;
}
//}}}
let distances:Array<number> = p1(flyers, keys);
let points:Array<number> = p2(flyers, keys);
console.log(`
P1: max distance = ${max(distances)}
P2: max points = ${max(points)}
`)
