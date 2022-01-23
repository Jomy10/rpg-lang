# The official RPG-lang compiler
[RPG](https://github.com/jomy10/rpg-lang) is an esoteric programming language

## Usage

```rust
use rpg_compiler::{compile, compile_with_config, Config};

// Use one of the compile functions
let output = compile("main.rpg");
let output = compile_with_config("main.rpg", Config { max_char: 10, verbose: false });
```

The variable output will contained the rust code of the rpg program. This can then be written to a file and compiled using cargo.

## Language Specification

### Actors
Actors are either a [character](#characters) or a [zombie](#zombies). They have 2 variables
called **health** and **attack**. They also have an inventory to hold [items](#items). The maximum amount of actors
allowed per game (e.g. per program) is **10**. Characters will disappear when they die (e.g. when their health is 0),
but zombies won't disappear when their health is 0 or below. They need to be [converted to a char](#un_zombify)

Characters can cast more [spells](#spells), but can't have negative health, while zombies can have negative health.

#### Characters
Characters have 2 variables **health** and **attack**, both are unsigned 32-bit integers.

A character called "ash" with health of 5 and attack of 3:

```
char ash = (5, 3)
```

#### Zombies
Zombies are actors that can have negative **health** (signed 32-bit integer). They can be converted to
a regular character using the [un_zombify](#un_zombify) spell.

```
zombie walker = (-4, 6)
```

### Items
#### Potions
Potions have 1 variable called **healing** and can be used to heal an actor. Actors buy these potions
from [merchants](#merchants) and need to buy them as many times as they will use the potion.

```
potion p = (5)
```

#### Spell books
Spell books are used for casting different [spells](#spells).

```
spellbook eucharidion = ()
```

### Merchants
Merchants sell items. Actors can buy these items and will hold them in their inventory.

```
merchant cabbage = ()
```

Actors buy items from merchants using `buys`

```
actor buys item from merchant
```

### Attacking
Actors can attack each other, this subtracts the attack of the attacking actor from the health of tha actor being attacked.

``` 
char a = (10, 3)
char b = (2, 5)

b attacks a
# a will now have 10 - 5 = 5 health
```

### Using items
Actors can use items in their inventory.

```
potion p = (5)
char a = (5,0)
merchant m = ()
a buys p from m

a uses p
# a will now have 5 + 5 = 10 health
```

### Outputting to the screen

#### Shouts
Characters can shout their health. This will output a new line.

```
char a = (1,0)
a shouts
a shouts
# output:
# 1
# 1
```

#### Whispers
Characters can whisper their health. This will output without a new line.

```
char a = (1,0)
a whispers
a whispers
# output: 
# 11
```

### Spells
Characters can use [spell books](#spell-books) to cast spells. They are casting using a spellbook and the spell name is
followed by `()` or `(param)`

### God_speech
This will read whatever number the user inputs

```
input uses spellbook casting god_speech()
```

#### Speak
This will print the ASCII value of the health of the actor

```
char jeremy = (33,1)
spellbook eucharidium = ()
merchant cabbage_man = ()

jeremy buys eucharidion from cabbage_man

jeremy shouts eucharidion casting speak()
# output:
# !
```

#### Time_warp
The time warp performs the lines beneath it until it reaches the `end` keyword, at which point it will go back to the
beginning and performs the lines again. To do this, it will require an offer. It will perform this loop until the offered
character has no health left.

The health of the character being consumed is subtracted at the end of the lines.

```
# We have 2 characters: david (5 health)  and ella (5 health). David has a spellbook in its inventories
david uses spellbook casting time_warp(ella)
	ella shouts
end

# Output:
# 5
# 4
# 3
# 2
# 1
```

### Un_zombify
Converts a character to a zombie.

```
james_brown uses spell_book casting un_zombify(zombie1)
```

### Confuse
When a character is confused, it will output its health - 1 when shouting or whispering.

```
steven uses spell_book casting confuse(other_char)
other_char shouts # Will output the other_char's health - 1 e.g. other_char = (1,2), so the output will be: 0
```

### Create_potion
Characters can change the value of a potion

```
char sans = (6, 1)
potion p = (5)
sans buys p from merchant

sans uses spellbook casting create_potion(p)
sans uses p
sans shouts
# Output:
# 12 (6 + 6)
```

### Shift
Shift swaps a character's health and attack.

```
char Ness = (2,1)
Ness uses spellbook casting shift()
Ness shouts
# Output:
# 1
```