# Combat system

## Concepts

All properties:
- HP (hitpoints)
- DA (damage)
- AT (armor toughness)
- AP (armor penetration)
- AR (damage reduction)
- AM (damage modifier)
- RA (range)
- AC (accuracy)
- EV (evasion)
- TR (tracking)

### Weapons
- Laser:
-- Range: Short
-- Cost: Cheap
-- Strength: Weak
-- AoA: Single-Target
-- AmmoDepth: Deep
-- canIntercept: Yes
-- Interceptable: No
- Kinetics: 
-- Range: Medium
-- Cost: Moderate
-- Strength: Moderate
-- AoA: Single-Target
-- AmmoDepth: Moderate
-- canIntercept: Yes
-- Interceptable: Yes
- Missiles: 
-- Range: Long
-- Cost: Expensive
-- Cost: Strong
-- AoA: Wide
-- AmmoDepth: Shallow
-- canIntercept: Yes
-- Interceptable: Yes

### Health x Armor x Damage

Each ship component in the designer attaches a maxHP amount.

Armor plating attaches an armor values.

Damage offsets against health, armor acts as linear health reduction (can reduce to zero).

Armor penetration offsets against armor.

Armor has 3 components: armor, DR, DM

toughness = max(armor - armorPenetration, 0)

nextHealth = max(health - max(damage - toughness, 0) - inelastic * max(min(damage, toughness) - elastic), 0)

Or, refactor as:

```c
hp(t+1) = max(hp - {
    toughness = max(armor - armorPen, 0)
    if (damage >= toughness) {
        damage - toughness * (1 + inelastic) + inelastic * elastic
    } else {
        inelastic * (damage - elastic)
    }
}, 0);
```

Shields don't exist.

Range is range of weapon on a battlefield, no logic is dedicated to any idea of minimum range or elevation/attitude, ships act as points that can fire in any direction at any time.

Evasion/Tracking are competing factors to hit. Base accuracy represents probability to hit if firing randomly, tracking increases to hit probability, evasion counters tracking.

ChanceToHit = min(1.0, baseAccuracy + max(Tracking - Evasion, 0))
