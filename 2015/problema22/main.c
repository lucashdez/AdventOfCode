#include <stdio.h>
#include "base/base.h"
#include "base/base_include.h"
#define MAX_INT

static struct MM_BaseMemory *GLOBAL_BASE_ALLOCATOR;

static int best_yet;

int main(int argc, char **argv) {
    GLOBAL_BASE_ALLOCATOR = mm_create_malloc_base_memory();
}
