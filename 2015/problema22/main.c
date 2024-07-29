#include <stdio.h>
#include "base/base.h"
#include "base/base_include.h"
#define MAX_INT

int best = 9999999;
b32 hard = true;

typedef struct _state {
    int mana, acc_mana, bhp, php;
    char shield_turns, poison_turns, recharge_turns; //Char cause char can only be > 0; 
} State;

void every_turn(State *s) {
    if (s->shield_turns > 0) {
        s->shield_turns -= 1;
    }
    if (s->poison_turns > 0) {
        s->poison_turns -= 1;
        s->bhp -= 3;
    }
    if (s->recharge_turns > 0) {
        s->recharge_turns -= 1;
        s->mana += 101;
    }
}

void player_turn(State s);

void boss_turn(State s) {
    every_turn(&s);
    s.php -= (s.shield_turns > 0)? 1 : 8;
    if (s.php > 0) {
        player_turn(s);
    }
}

void magic_missile(State s) {
    s.bhp -= 4;
    if (s.bhp <= 0) {
        best = Min(best, s.acc_mana);
    } else {
        boss_turn(s);
    }
}

void drain(State s) {
    s.bhp -= 2;
    s.php += 2;
    if (s.bhp <= 0) {
        best = Min(best, s.acc_mana);
    } else {
        boss_turn(s);
    }
}

void shield(State s) {
    s.shield_turns += 6;
    boss_turn(s);
}

void poison(State s) {
    s.poison_turns += 6;
    boss_turn(s);
}

void recharge(State s) {
    s.recharge_turns += 5;
    boss_turn(s);
}


struct _spells {
    int cost;
    void (*f)(State); 
} spells[] = {
    {53, magic_missile},
    {73, drain},
    {113, shield},
    {173, poison},
    {229, recharge},
};

void player_turn(State s) {
    if (hard) {
        s.php -= 1;
        if (s.php <= 0) {
            return;
        }
    }
    every_turn(&s);
    if (s.bhp <= 0) {
        best = Min(best, s.acc_mana);
        return;
    }
    for (int i = 0;
        i < 5;
        ++i) {
        if (s.mana >= spells[i].cost && (s.acc_mana + spells[i].cost) < best) {
            s.mana -= spells[i].cost;
            s.acc_mana += spells[i].cost;
            spells[i].f(s);
            s.mana += spells[i].cost;
            s.acc_mana -= spells[i].cost;
        } else {
            return;
        }
    }

}

int main(int argc, char **argv) {
    State s = {.acc_mana = 0, .mana = 500, .bhp = 55, .php = 50, .poison_turns = 0, .shield_turns = 0, .recharge_turns = 0};
    player_turn(s);
    printf("best: %d", best);
    return 0;
}
