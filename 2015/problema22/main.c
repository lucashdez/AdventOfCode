#include <stdio.h>
#include "base/base_include.h"
#define MINF 1000

static struct MM_BaseMemory *GLOBAL_BASE_ALLOCATOR;
typedef struct _edge Edge;

typedef struct _node {
	i32 php;
	i32 bhp;
	i32 m;
	i32 poison;
	i32 shield;
	i32 recharge;
	b32 is_win_path;
	Edge *edges;
} Node;

typedef struct _edge {
	Node* n;
	i32 c;
} Edge;

typedef struct _graph {
	Node* first;
} Graph;

int possible_paths(int mana) {
	return (mana > 253)? 5 : (mana > 173)? 4 : (mana);
}

Edge* generate_edges(Node* node) {
	int til = possible_paths(node->m);
	return 0;
}

void print_node(Node* n) {
	printf("php {%d}, bhp {%d}, m {%d}, poison {%d}, shield {%d}, recharge {%d}",
	n->php,
	n->bhp,
	n->m = 250,
	n->poison = 0,
	n->shield = 0,
	n->recharge = 0);
}

int main(int argc, char **argv) {
	GLOBAL_BASE_ALLOCATOR = mm_create_malloc_base_memory();
	struct Arena global_arena = mm_make_arena_reserve(GLOBAL_BASE_ALLOCATOR ,MB(1));
	Node* first = MMPushArrayZeros(&global_arena, Node, 1);
	first->php = 10;
	first->bhp = 13;
	first->m = 250;
	first->poison = 0;
	first->shield = 0;
	first->recharge = 0;
	first->is_win_path = true;
	first->edges = 0;
	Graph graph = {first};
	print_node(graph.first);
}
