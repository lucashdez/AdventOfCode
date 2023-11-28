/**
* Weapons:    Cost  Damage  Armor
* Dagger        8     4       0
* Shortsword   10     5       0
* Warhammer    25     6       0
* Longsword    40     7       0
* Greataxe     74     8       0
*
* Armor:      Cost  Damage  Armor
* Leather      13     0       1
* Chainmail    31     0       2
* Splintmail   53     0       3
* Bandedmail   75     0       4
* Platemail   102     0       5
*
* Rings:      Cost  Damage  Armor
* Damage +1    25     1       0
* Damage +2    50     2       0
* Damage +3   100     3       0
* Defense +1   20     0       1
* Defense +2   40     0       2
* Defense +3   80     0       3
*/

class Item {
	/**
	 * Weapon: 0
	 * Armor: 1
	 * Rings: 2
	 */
	public category = -1; 
	constructor() {
	}
}

class Combo {
	constructor(weapon, armor, r1, r2, cost) {
	}
}

class Market {
	public combos: Array<Combo> = [];
	public weapons: Array<Item> = [];
	public armors: Array<Item> = [];
	public rings: Array<Item> = [];
	constructor() {
	}
}




function P1(shop: Market) {
}

function P2() {
}

function main() {
	const shop = new Market;
	let p1 = P1(shop);
	let p2 = P2();
	console.log(`P1: ${p1}\nP2: ${p2}`)
}
