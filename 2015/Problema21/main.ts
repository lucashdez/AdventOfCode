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

	public name = "";
	/** Weapon: 0 Armor: 1 Rings: 2 */
	public category = -1; 
	public cost = 0;
	public damage = 0;
	public armor = 0;
	constructor(s: string, type: number, c: number, dmg: number, def: number) {
		this.name = s;
		this.category = type;
		this.cost = c;
		this.damage = dmg;
		this.armor = def;
	}
}

class Combo {
	public weapon;
	public armor;
	public ring1;
	public ring2;
	public cost
	constructor(w: Item, a: Item, r1: Item, r2: Item, c: number) {
        this.weapon = w;
		this.armor = a;
		this.ring1 = r1;
		this.ring2 = r2;
		this.cost = w.cost + a.cost + r1.cost + r2.cost;
	}
}

class Market {
	public combos: Array<Combo> = [];
	public weapons: Array<Item> = [];
	public armors: Array<Item> = [];
	public rings: Array<Item> = [];
	public generate_combos() {
		for (let wi = 0; wi < this.weapons.length; ++wi) {
			for (let ai = 0; ai < this.armors.length; ++ai) {
				for (let ri1 = 0; ri1 < this.rings.length; ++ri1) {
					for(let ri2 = ri1; ri2 < this.rings.length; ++ri2) {
					} 
				}
			}
		}
	
	}

	constructor() {
		this.weapons = [
			//COLUMNS              t  c   d  a
			new Item("Dagger",     0, 8,  4, 0),
			new Item("Shortsword", 0, 10, 5, 0),
			new Item("Warhammer",  0, 25, 6, 0),
			new Item("Longsword",  0, 40, 7, 0),
			new Item("Greataxe",   0, 74, 8, 0)
		]
		this.armors = [
			//COLUMNS              t  c    d  a
			new Item("Leather",    1, 13,  0, 1),
			new Item("Chainmail",  1, 31,  0, 2),
			new Item("Splitmail",  1, 53,  0, 3),
			new Item("Bandedmail", 1, 75,  0, 4),
			new Item("Platemail",  1, 102, 0, 5)
		]
		this.rings = [
			//COLUMNS              t   c    d  a
			new Item("Damage +1",  2,  25,  1,  0),
			new Item("Damage +2",  2,  50,  2,  0),
			new Item("Damage +3",  2, 100,  3,  0),
			new Item("Defense +1", 2,  20,  0,  1),
			new Item("Defense +1", 2,  40,  0,  2),
			new Item("Defense +1", 2,  80,  0,  3)
		]
	}
}

function P1(shop: Market) {
}

function P2() {
}

function main() {
	const shop = new Market;
	shop.generate_combos();
	let p1 = P1(shop);
	let p2 = P2();
	console.log(`P1: ${p1}\nP2: ${p2}`)
}
