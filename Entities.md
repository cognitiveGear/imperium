# List of Entity Types

Players
- targetOf Stock
- targetOf Modifier

Locations

Stars
- isA Location

Hyperlanes
- isA Location
- targets Star (start)
- targets Star (end)

Planets
- isA Location
- childOf Star

Deposits
- IsA Location
- childOf Planet

Species
- targets Designer (Player)
- targetOf RaceTraits

Pops
- targets Owner (Player)
- targets Location (Location)
- targets Species
- targetOf Upkeep
- targetOf Modifier
- targetOf Build
- has Gender
- has Job

Habs
- targets Owner (Player)
- targets Location (Location)
- targetOf Build
- targetOf Upkeep
- has HabType

Ships
- isA Location
- targets Owner (Player)
- targets ParentLocation (Location)
- targetOf Build
- targetOf Upkeep
- targetOf Stock

## Types

Resource (enum)
Tech (enum)
HabType (enum) -- Actual structure type
HabClass (enum) -- Where hab is (Surface, Orbit, Ship)
Job (enum)
Traits (enum)
Gender (enum)

## Linking Entities

Build (Res, quantity (float))
Stock (Res, quantity (float))
Upkeep (Res, quantity (float))
Modifier (Res, modifier (float))
RaceTraits (Trait)
