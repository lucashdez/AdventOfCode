var ingredients = {
    Sprinkles: {
        capacity: 5,
        durability: -1,
        flavor: 0,
        texture: 0,
        calories: 5,
    },
    "Peanut Butter": {
        capacity: -1,
        durability: 3,
        flavor: 0,
        texture: 0,
        calories: 1,
    },
    Frosting: {
        capacity: 0,
        durability: -1,
        flavor: 4,
        texture: 0,
        calories: 6,
    },
    Sugar: {
        capacity: -1,
        durability: 0,
        flavor: 0,
        texture: 2,
        calories: 8,
    },
};

var ingredientNames = Object.keys(ingredients);

function getRemainderPossibilities(total, n) {
    n = n || 0;
    var spaces = new Array(n * 4 + 1).join(" ");

    var possibilities = [];

    if (n === ingredientNames.length - 1) {
        return [
            [
                {
                    name: ingredientNames[n],
                    amount: total,
                },
            ],
        ];
    } else {
        for (var i = total; i >= 0; i--) {
            var item = {
                name: ingredientNames[n],
                amount: i,
            };

            if (i !== total) {
                var remainder = getRemainderPossibilities(total - i, n + 1);
                if (!remainder.length) {
                    console.log(spaces, "debg:", total - i, n + 1);
                }
                remainder.forEach(function (list) {
                    if (i !== 0) {
                        list.unshift(item);
                    }
                    possibilities.push(list);
                });
            } else {
                possibilities.push([item]);
            }
        }
    }

    return possibilities;
}

function score(list, requiredCalories) {
    let capacity = 0;
    let durability = 0;
    let flavor = 0;
    let texture = 0;
    let calories = 0;

    for (var i = 0; i < list.length; i++) {
        var item = list[i];
        capacity += ingredients[item.name].capacity * item.amount;
        durability += ingredients[item.name].durability * item.amount;
        flavor += ingredients[item.name].flavor * item.amount;
        texture += ingredients[item.name].texture * item.amount;
        calories += ingredients[item.name].calories * item.amount;
    }

    if (capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0)
        return 0;

    if (requiredCalories && calories !== requiredCalories) return 0;

    return capacity * durability * flavor * texture;
}

var possibilities = getRemainderPossibilities(100);
let partOne = 0;
let partTwo = 0;
possibilities.forEach(function (list, i) {
    partOne = Math.max(partOne, score(list));
    partTwo = Math.max(partTwo, score(list, 500));
});

console.log("Part One:", partOne);
console.log("Part Two:", partTwo);
