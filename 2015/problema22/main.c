#include <stdio.h>
#include "base/base_include.h"

static struct MM_BaseMemory *GLOBAL_BASE_ALLOCATOR;


typedef struct _effects {
	i32 poison;
	i32 shield;
	i32 recharge;
} Effects;


typedef struct _Status {
	int php;
	int bhp;
	int mana;
	int bdmg;
	Effects e;
	i8 spells;
}Status;



typedef struct _listChilds {
	struct _Node *first;
	struct _Node *last;
} ListChilds;


typedef struct _Node {
	struct Arena arena;
	int level;
	Status s;
	struct _Node* next;
	ListChilds childs;
	b32 end;
	b32 t;
}Node ;


struct Tree {
	struct Arena arena;
	Node father;
	int levels;
};


// 00000
i8 get_possible_spells(int mana) {
	i8 result = 0;
	if (mana >= 229) 
		result |= 16;
	if (mana >= 173)
		result |= 8;
	if (mana >= 113)
		result |= 4;
	if (mana >= 73)
		result |= 2;
	if (mana >= 53)
		result |= 1;
	return(result);
}

// Functions


void l(Node n) {
	char c[6];
	c[0] = (n.s.spells & 16)? '1' : '0'; 
	c[1] = (n.s.spells & 8)? '1' : '0'; 
	c[2] = (n.s.spells & 4)? '1' : '0'; 
	c[3] = (n.s.spells & 2)? '1' : '0'; 
	c[4] = (n.s.spells & 1)? '1' : '0'; 
	c[5] ='\0';
	printf("%d: %d -> %s\n", n.level, n.s.mana, c);
}

struct NodeList {
	struct Arena arena;
	Node* first;
	Node* last;
	Node* next;
};


Node* magic_missile(Node *p) {
	Node *r = MMPushArrayZeros(&p->arena, Node, 1);
	r->arena = mm_scratch_arena();
	r->childs = (ListChilds){0};
	r->level = p->level + 1;
	r->s.php = p->s.php; 
	r->s.bhp = p->s.bhp - 4;
	r->s.mana = p->s.mana - 53;
	r->s.bdmg = p->s.bdmg;
	r->s.e = p->s.e;
	r->end = false;
	r->t = !p->t;
	sll_queue_push(p->childs.first, p->childs.last, r);
	return r;
}

Node *drain(Node *p) {
	Node *r = MMPushArrayZeros(&p->arena, Node, 1);
	r->arena = mm_scratch_arena();
	r->childs = (ListChilds){0};
	r->level = p->level + 1;
	r->s.php = p->s.php + 2; 
	r->s.bhp = p->s.bhp - 2;
	r->s.mana = p->s.mana - 73;
	r->s.bdmg = p->s.bdmg;
	r->s.e = p->s.e;
	r->end = false;
	r->t = !p->t;
	sll_queue_push(p->childs.first, p->childs.last, r);
	return r;
}

Node *shield(Node *p) {
	Node *r = MMPushArrayZeros(&p->arena, Node, 1);
	r->arena = mm_scratch_arena();
	r->childs = (ListChilds){0};
	r->level = p->level + 1;
	r->s.php = p->s.php; 
	r->s.bhp = p->s.bhp;
	r->s.mana = p->s.mana - 113;
	r->s.bdmg = p->s.bdmg;
	r->s.e = p->s.e;
	r->s.e.shield = p->s.e.shield + 6;
	r->end = false;
	r->t = !p->t;
	sll_queue_push(p->childs.first, p->childs.last, r);
	return r;
}

Node *poison(Node *p) {
	Node *r = MMPushArrayZeros(&p->arena, Node, 1);
	r->arena = mm_scratch_arena();
	r->childs = (ListChilds){0};
	r->level = p->level + 1;
	r->s.php = p->s.php; 
	r->s.bhp = p->s.bhp;
	r->s.mana = p->s.mana - 173;
	r->s.bdmg = p->s.bdmg;
	r->s.e = p->s.e;
	r->s.e.poison = p->s.e.poison + 6;
	r->end = false;
	r->t = !p->t;
	sll_queue_push(p->childs.first, p->childs.last, r);
	return r;
}

Node *recharge(Node *p) {
	Node *r = MMPushArrayZeros(&p->arena, Node, 1);
	r->arena = mm_scratch_arena();
	r->childs = (ListChilds){0};
	r->level = p->level + 1;
	r->s.php = p->s.php; 
	r->s.bhp = p->s.bhp;
	r->s.mana = p->s.mana - 229;
	r->s.bdmg = p->s.bdmg;
	r->s.e = p->s.e;
	r->s.e.recharge = p->s.e.recharge + 5;
	r->end = false;
	r->t = !p->t;
	sll_queue_push(p->childs.first, p->childs.last, r);
	return r;
}

void apply_effects(Node *n) {
	n->s.bdmg = (n->s.e.shield > 0)?    1 : 8;
	n->s.bhp -= (n->s.e.poison > 0)?    3 : 0;
	n->s.mana += (n->s.e.recharge > 0)? 101 : 0;

	n->s.e.shield -= (n->s.e.shield-1 < 0)? 0 : 1;
	n->s.e.poison -= (n->s.e.poison-1 < 0)? 0 : 1;
	n->s.e.recharge -= (n->s.e.recharge-1 < 0)? 0 : 1;
}

Node* boss_attack(Node *p) {
	Node *r = MMPushArrayZeros(&p->arena, Node, 1);
	r->arena = mm_scratch_arena();
	r->childs = (ListChilds){0};
	r->level = p->level + 1;
	r->s.php = p->s.php - p->s.bdmg; 
	r->s.bhp = p->s.bhp;
	r->s.mana = p->s.mana;
	r->s.bdmg = p->s.bdmg;
	r->s.e = p->s.e;
	r->end = false;
	r->t = !p->t;
	sll_queue_push(p->childs.first, p->childs.last, r);
	return r;
}

void has_ended(Node* n) {
	if (n->s.php <= 0 || n->s.bhp <= 0 || n->s.mana < 53) {
		n->end = true;
	}
}

void attack(struct NodeList* l, Node *n)  {
	if (n->s.spells & 16) 
		sll_queue_push(l->first, l->last, recharge(n));
	if (n->s.spells & 8)
		sll_queue_push(l->first, l->last, poison(n));
	if (n->s.spells & 4)
		sll_queue_push(l->first, l->last, shield(n));
	if (n->s.spells & 2)
		sll_queue_push(l->first, l->last, drain(n));
	if (n->s.spells & 1)
		sll_queue_push(l->first, l->last, magic_missile(n));
}

void game(struct Tree* tree) {
	struct NodeList list = {0};
	Node *current = &tree->father;
	b32 start = true;
	
	while(current != 0 || start) {
		start = false;
		apply_effects(current);
		current->s.spells = get_possible_spells(current->s.mana);
		has_ended(current);
		if (!current->end) {
			if (current->t) {
				attack(&list, current);
				has_ended(current);
			} else {
				sll_queue_push(list.first, list.last, boss_attack(current));
				has_ended(current);
			}
			current = list.first;
			sll_queue_pop(list.first, list.last);
		} else {
			current = list.first;
			sll_queue_pop(list.first, list.last);
		}
	}
}

int main(int argc, char **argv) {
	struct Tree tree = {0};
	GLOBAL_BASE_ALLOCATOR = mm_create_malloc_base_memory();
	tree.arena = mm_make_arena_reserve(GLOBAL_BASE_ALLOCATOR, KB(64));
	Status init_status = {10, 13, 250, 8, {0,0,0}, get_possible_spells(250)};
	Node first =  {mm_scratch_arena(), 0, init_status, 0, {0},false,true};
	ListChilds childs = {0};
	tree.father = first;
	tree.levels = 0;
	tree.father.childs = childs;
	l(tree.father);
	game(&tree);
}
