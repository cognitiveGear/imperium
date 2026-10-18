# List of Entity Types

Players
- targetOf Stock
- targetOf Modifier
- has AI

Location

Located
- targets Location

Star
- isA Location

Hyperlanes
- isA Location
- targets Star (start)
- targets Star (end)

Planet
- isA Location
- childOf Star

Deposits
- isA Location
- childOf Planet 
- has DepositType

Designed
- has Designer (Player)

Species
- isA Designed
- targetOf RaceTraits
- targetOf Build
- targetOf Upkeep
- targetOf Modifier
- has Name
- has Description
- has Sprite

ShipDesign
- isA Designed
- targetOf Build
- targetOf Upkeep
- targetOf Stock
- has Sprite

Owned
- has Owner (Player)

Pops
- isA Owned
- isA Located
- targets Species
- has Gender
- has Job
- has AI

Habs
- isA Owned
- isA Located
- targetOf Build
- targetOf Upkeep
- has HabType

Ships
- isA Owned
- isA Location
- isA Located
- targets ShipDesign
- has Position
- has Velocity
- has HP
- has AI

## Types

- Resource (enum)
- Tech (enum)
- HabType (enum)
Actual structure type, associate res build/upkeep
- HabClass (enum)
Where hab can be located (Surface, Orbit, Ship)
- Job (enum)
- Traits (enum)
Associated with res build/upkeep
- Gender (enum)
- DepositType (enum)

## Linking Entities

- Build (Res, quantity (float))
- Stock (Res, quantity (float))
- Upkeep (Res, quantity (float))
- Modifier (Res, modifier (float))
- RaceTraits (Trait)
