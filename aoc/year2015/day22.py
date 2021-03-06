#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
https://adventofcode.com/2015/day/22
https://www.reddit.com/r/adventofcode/comments/3xspyl/day_22_solutions/cy927kk?
utm_source=share&utm_medium=web2x&context=3
"""


from collections import namedtuple
from heapq import heappop, heappush
from itertools import count
from functools import reduce
import re

from data.utils import get_input


SPELL = namedtuple("BaseSpell", "name cost effect turns dmg heal armor mana")


class Spell(SPELL):
    def __new__(cls, name, cost, effect=0, turns=None, dmg=0, heal=0, armor=0, mana=0):
        packed = (cls, name, cost, effect, turns, dmg, heal, armor, mana)
        return super().__new__(*packed)


spells = (
    Spell("Magic Missile", 53, dmg=4),
    Spell("Drain", 73, dmg=2, heal=2),
    Spell("Shield", 113, effect=True, turns=6, armor=7),
    Spell("Poison", 173, effect=True, turns=6, dmg=3),
    Spell("Recharge", 229, effect=True, turns=5, mana=101),
)


class State(object):
    def __init__(
        self,
        hp,
        mana,
        boss_hp,
        boss_dmg,
        mana_spent=0,
        effects=None,
        hard=False,
        parent=None,
        spell_cast=None,
    ):
        self.hp = hp
        self.mana = mana
        self.boss_hp = boss_hp
        self.boss_dmg = boss_dmg
        self.mana_spent = mana_spent
        self.effects = effects or ()
        self.hard = hard
        self._parent = parent
        self._spell_cast = spell_cast

    def __eq__(self, other):
        if not isinstance(other, State):
            return NotImplemented
        filtered = filter(lambda k: k[0] != "_", vars(self))
        return all(getattr(self, k) == getattr(other, k) for k in filtered)

    def __hash__(self):
        return reduce(
            lambda a, b: a ^ hash(b),
            (v for k, v in vars(self).items() if k[0] != "_"),
            0,
        )

    def iter_path(self):
        if self._parent is None:
            return
        yield from self._parent.iter_path()
        yield self._spell_cast

    def process_effects(self, effects, hp, mana, boss_hp):
        remaining_effects = []
        armor = 0  # either Shield is in effect or it is not
        for timer, effect in self.effects:
            hp += effect.heal
            mana += effect.mana
            boss_hp -= effect.dmg
            armor = max(armor, effect.armor)
            if timer > 1:
                remaining_effects.append((timer - 1, effect))
        return tuple(remaining_effects), hp, mana, boss_hp, armor

    def boss_turn(self):
        packed = (self.effects, self.hp, self.mana, self.boss_hp)
        effects = self.process_effects(*packed)
        self.effects, self.hp, self.mana, self.boss_hp, armor = effects
        # only if the boss is still alive can they attack!
        if self.boss_hp > 0:
            self.hp -= max(1, self.boss_dmg - armor)

    def transitions(self):
        # Player turn first
        effects, hp, mana, boss_hp, __ = self.process_effects(
            self.effects, self.hp - int(self.hard), self.mana, self.boss_hp
        )
        for spell in spells:
            if spell.cost > mana or any(spell is s for t, s in effects):
                # can't cast spells for which we have no mana or in effect
                continue
            new_state = State(
                hp,
                mana - spell.cost,
                boss_hp,
                self.boss_dmg,
                self.mana_spent + spell.cost,
                effects,
                hard=self.hard,
                parent=self,
                spell_cast=spell.name,
            )
            if not spell.effect:
                new_state.hp += spell.heal
                new_state.boss_hp -= spell.dmg
            else:
                new_state.effects = new_state.effects + ((spell.turns, spell),)
            # Boss turn next
            new_state.boss_turn()
            # No point in playing a turn that has the player losing
            if new_state.hp > 0:
                yield new_state


def search_a_star(start):
    open_states = {start}
    pqueue = [(0, start)]
    closed_states = set()
    unique = count()
    while open_states:
        current = heappop(pqueue)[-1]
        if current.boss_hp < 1:
            return current
        open_states.remove(current)
        closed_states.add(current)
        for state in current.transitions():
            if state in closed_states or state in open_states:
                continue
            open_states.add(state)
            heappush(pqueue, (state.mana_spent, next(unique), state))


def part1(text, p_health=50, p_mana=500):
    b_health, b_attack = map(int, re.findall(r"\d+", text))
    start = State(p_health, p_mana, b_health, b_attack)
    return search_a_star(start).mana_spent


def part2(text, p_health=50, p_mana=500):
    b_health, b_attack = map(int, re.findall(r"\d+", text))
    start = State(p_health, p_mana, b_health, b_attack)
    start.hard = True
    return search_a_star(start).mana_spent


if __name__ == "__main__":  # pragma: no cover
    text = get_input(2015, 22)

    print(part1(text))
    print(part2(text))
