"use strict";
exports.__esModule = true;
var fs_1 = require("fs");
//{{{ Lights Init
var lArr = [];
for (var i = 0; i < 1000; ++i) {
    var lights = [];
    for (var j = 0; j < 1000; ++j) {
        lights.push(0);
    }
    lArr.push(lights);
}
//}}}
var data = (0, fs_1.readFileSync)('./input.txt', 'utf-8');
var dataArr = data.split('\n');
dataArr = dataArr.filter(function (d) { return d !== ''; });
for (var _i = 0, dataArr_1 = dataArr; _i < dataArr_1.length; _i++) {
    var order = dataArr_1[_i];
    var oArr = order.split(' ');
    var i1 = void 0;
    var i2 = void 0;
    var f1 = void 0;
    var f2 = void 0;
    var command = void 0;
    if (oArr.length === 4) {
        var ini = oArr[1].split(',');
        i1 = Number(ini[0]);
        i2 = Number(ini[1]);
        var fin = oArr[3].split(',');
        f1 = Number(fin[0]);
        f2 = Number(fin[1]);
        command = 'toggle';
    }
    else {
        command = oArr[1];
        var ini = oArr[2].split(',');
        i1 = Number(ini[0]);
        i2 = Number(ini[1]);
        var fin = oArr[4].split(',');
        f1 = Number(fin[0]);
        f2 = Number(fin[1]);
    }
    for (var i = i1; i <= f1; ++i) {
        for (var j = i2; j <= f2; ++j) {
            switch (command) {
                case 'toggle':
                    lArr[i][j] += 2;
                    break;
                case 'on':
                    lArr[i][j] += 1;
                    break;
                case 'off':
                    lArr[i][j] -= (lArr[i][j] > 0) ? 1 : 0;
                    break;
                default:
                    console.log('WTFWTF');
            }
        }
    }
}
var counter = 0;
for (var i = 0; i < 1000; ++i)
    for (var j = 0; j < 1000; ++j)
        counter += lArr[i][j];
console.log(counter);
