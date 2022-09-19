"use strict";
exports.__esModule = true;
var fs = require("fs");
var data = fs.readFileSync('input.txt', 'utf-8').split('\n').filter(function (d) { return d !== ''; });
var re = /(?<NAME>[a-zA-Z]+) can fly (?<SPEED>\d+) km\/s for (?<LIMIT>\d+) seconds, but then must rest for (?<REST>\d+) seconds./;
var flyers = Object.create(null);
//{{{ INIT
data.forEach(function (s) {
    var result = re.exec(s).groups;
    flyers[result.NAME] = {
        distance: 0,
        wait: 0,
        speed: parseInt(result.SPEED),
        limit: parseInt(result.LIMIT),
        left: parseInt(result.LIMIT),
        rest: parseInt(result.REST),
        points: 0
    };
});
//}}}
//{{{ MAX
var max = function (arr) {
    var max = 0;
    arr.forEach(function (e) {
        if (max < e)
            max = e;
    });
    return max;
};
//}}}
var keys = Object.keys(flyers);
//{{{ p1(flyers, keys)
var p1 = function (flyers, keys) {
    for (var n = 0; n < 2503; n++) {
        keys.forEach(function (key) {
            if (flyers[key].wait === 0) {
                flyers[key].distance += flyers[key].speed;
                flyers[key].left--;
                if (flyers[key].left === 0) {
                    flyers[key].wait = flyers[key].rest;
                    flyers[key].left = flyers[key].limit;
                }
            }
            else {
                flyers[key].wait--;
            }
        });
    }
    var distances = [];
    keys.forEach(function (key) {
        distances.push(flyers[key].distance);
        flyers[key].distance = 0;
        flyers[key].wait = 0;
        flyers[key].left = flyers[key].limit;
    });
    return distances;
};
//}}}
//{{{ P2
var p2 = function (flyers, keys) {
    var _loop_1 = function (n) {
        var c_max = { value: 0, key: "" };
        keys.forEach(function (key) {
            if (flyers[key].wait === 0) {
                flyers[key].distance += flyers[key].speed;
                flyers[key].left--;
                if (flyers[key].left === 0) {
                    flyers[key].wait = flyers[key].rest;
                    flyers[key].left = flyers[key].limit;
                }
            }
            else {
                flyers[key].wait--;
            }
            if (c_max.value < flyers[key].distance) {
                c_max.key = key;
                c_max.value = flyers[key].distance;
            }
        });
        keys.forEach(function (key) {
            if (c_max.value === flyers[key].distance) {
                flyers[key].points++;
            }
        });
        c_max.key = "";
        c_max.value = 0;
    };
    for (var n = 0; n < 2503; ++n) {
        _loop_1(n);
    }
    var points = [];
    keys.forEach(function (key) {
        points.push(flyers[key].points);
    });
    return points;
};
//}}}
var distances = p1(flyers, keys);
var points = p2(flyers, keys);
console.log("\nP1: max distance = ".concat(max(distances), "\nP2: max points = ").concat(max(points), "\n"));
