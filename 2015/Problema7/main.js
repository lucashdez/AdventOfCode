"use strict";
exports.__esModule = true;
var fs = require("fs");
var input = fs.readFileSync('input.txt', 'utf-8');
var data = input.split('\n');
data = data.filter(function (d) { return d !== ''; });
var wires = {};
var Wire = /** @class */ (function () {
    function Wire(instruction) {
        this.value = undefined;
        this.calculate = null;
        this.ops = {
            AND: '&',
            OR: '|',
            LSHIFT: '<<',
            RSHIFT: '>>'
        };
        this.calculate = this.generateValueGetter(instruction);
    }
    Wire.prototype.getValue = function () {
        if (this.value === undefined) {
            this.value = this.checkRange(this.calculate());
        }
        return this.value;
    };
    Wire.prototype.checkRange = function (i) {
        var n = 65536;
        return ((i % n) + n) % n;
    };
    Wire.prototype.generateValueGetter = function (instruction) {
        var assignMatch, opMatch;
        if (assignMatch = /^(NOT )?([0-9]+|[a-z]+)$/.exec(instruction)) {
            return function () {
                var value = parseValue(assignMatch[2]);
                if (assignMatch[1])
                    value = ~value;
                return value;
            };
        }
        else if (opMatch = /^([a-z]+|[0-9]+) (AND|OR|LSHIFT|RSHIFT) ([a-z]+|[0-9]+)$/.exec(instruction)) {
            var opCode_1 = this.ops[opMatch[2]];
            return function () {
                return eval(parseValue(opMatch[1]) + ' ' + opCode_1 + ' ' + parseValue(opMatch[3]));
            };
        }
    };
    return Wire;
}());
function parseValue(key) {
    var i = parseInt(key);
    return !isNaN(i) ? i : wires[key].getValue();
}
data.forEach(function (item) {
    var match;
    if (match = /(.*) -> ([a-z]+)/.exec(item)) {
        wires[match[2]] = new Wire(match[1]);
    }
});
var order = Object.keys(wires).sort();
var p1 = wires.a.getValue();
for (var _i = 0, order_1 = order; _i < order_1.length; _i++) {
    var key = order_1[_i];
    wires[key].value = undefined;
}
wires.b.value = p1;
var p2 = wires.a.getValue();
console.log("\np1: ".concat(p1, ";\np2: ").concat(p2, "\n"));
