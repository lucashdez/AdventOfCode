"use strict";
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
exports.__esModule = true;
var fs = require("fs");
var data = fs.readFileSync('input.txt', 'utf-8').split('\n').filter(function (d) { return d !== ''; });
//Alice would gain 54 happiness units by sitting next to Bob.
//Alice would lose 79 happiness units by sitting next to Carol.
var re = /(?<NAME>[a-zA-Z]+) would (?<ACTION>(gain|lose)) (?<VALUE>\d+) happiness units by sitting next to (?<CONDITION>[a-zA-Z]+)./;
var names = new Set();
var relations = Object.create(null);
data.forEach(function (s) {
    var result = re.exec(s).groups;
    names.add(result.NAME);
    relations[result.NAME] = relations[result.NAME] || {};
    relations[result.NAME][result.CONDITION] =
        (result.ACTION === 'gain') ? parseInt(result.VALUE) :
            -parseInt(result.VALUE);
});
var aNames = [];
names.forEach(function (n) { return aNames.push(n); });
//{{{ permute
var permute = function (arr) {
    if (arr === void 0) { arr = []; }
    var res = [];
    var helper = function (arr2) {
        if (arr2.length == arr.length)
            return res.push(arr2);
        for (var _i = 0, arr_1 = arr; _i < arr_1.length; _i++) {
            var e = arr_1[_i];
            if (!arr2.includes(e))
                helper(__spreadArray(__spreadArray([], arr2, true), [e], false));
        }
    };
    helper([]);
    return res;
};
//}}}
var p = permute(aNames);
var nres = [];
p.forEach(function (aNames) {
    var nrel = 0;
    for (var i = 0; i < aNames.length; ++i) {
        if (i === aNames.length - 1) {
            nrel += relations[aNames[i]][aNames[0]] +
                relations[aNames[0]][aNames[i]];
        }
        else {
            nrel += relations[aNames[i]][aNames[i + 1]] +
                relations[aNames[i + 1]][aNames[i]];
        }
    }
    nres.push(nrel);
});
//{{{ min
var min = function (arr) {
    var min = 9999999999999;
    for (var _i = 0, arr_2 = arr; _i < arr_2.length; _i++) {
        var n = arr_2[_i];
        if (n < min)
            min = n;
    }
    return min;
};
//}}}
//{{{max 
var max = function (arr) {
    var max = -99999999999;
    for (var _i = 0, arr_3 = arr; _i < arr_3.length; _i++) {
        var n = arr_3[_i];
        if (n > max)
            max = n;
    }
    return max;
};
//}}}
console.log("\nP1: ".concat(max(nres), ";\n"));
