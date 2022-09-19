import * as fs from 'fs';

const input = fs.readFileSync('input.txt', 'utf-8');
let data = input.split('\n')
data = data.filter(d => d !== '')

let wires = {};

class Wire {
  value = undefined;
  calculate = null;
  ops = {
    AND:    '&',
    OR:     '|',
    LSHIFT: '<<',
    RSHIFT: '>>',
  }
  constructor(instruction:string) {
    this.calculate = this.generateValueGetter(instruction);
  }
  getValue() {
    if (this.value === undefined) {
      this.value = this.checkRange(this.calculate())
    }
    return this.value;
  }

  checkRange(i) {
    var n = 65536;
    return ((i % n) + n) % n;
  }
  
  generateValueGetter(instruction) {
    let assignMatch, opMatch;

    if(assignMatch = /^(NOT )?([0-9]+|[a-z]+)$/.exec(instruction)) {
      return function() {
        let value = parseValue(assignMatch[2]);
        if (assignMatch[1])
          value = ~value;
        return value;
      }
    } else if(opMatch = /^([a-z]+|[0-9]+) (AND|OR|LSHIFT|RSHIFT) ([a-z]+|[0-9]+)$/.exec(instruction)) {
      let opCode = this.ops[opMatch[2]];
      return function () {
        return eval(parseValue(opMatch[1]) + ' ' + opCode + ' ' + parseValue(opMatch[3]));
      }
    }
  }
}

function parseValue(key) {
  let i = parseInt(key);
  return !isNaN(i)?  i : wires[key].getValue();
}

data.forEach((item) => {
  let match;
  if (match = /(.*) -> ([a-z]+)/.exec(item)){
    wires[match[2]] = new Wire(match[1]);
  }
});

const order = Object.keys(wires).sort();

let p1 = wires.a.getValue();

for(let key of order) {
  wires[key].value = undefined;
}

wires.b.value = p1;

let p2 = wires.a.getValue()
console.log(`
p1: ${p1};
p2: ${p2}
`)
