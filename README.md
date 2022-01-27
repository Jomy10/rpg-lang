# The RPG esoteric programming language

The rpg language is a compiled esoteric programming language that writes like an rpg game.

## Table of contents
- [Download](#download)
- [Contributing](#contributing)
- [Using the cli](#using-the-cli)
- [Examples](#examples)
- [Bugs](#bugs)
- [Contributing](#contributing)
- [Specification](#language-specification)
- [Questions](#questions)
- [Other links](#other-links)
- [License](#license)

## Download
You will need the Rust compiler to compile this language alongside the rpgc cli.
You can download the Rust compiler from [rust-lang.org](https://www.rust-lang.org/tools/install).
But, in short, run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` on Mac or Linux.
For Windows, you can download a [standalone installer](https://forge.rust-lang.org/infra/other-installation-methods.html#standalone-installers).

You can then download the rpgc cli manually:
- [MacOS](https://github.com/Jomy10/rpg-lang/releases/download/v0.1.1/macos-x86_64.tar.gz)
- [Linux](https://github.com/Jomy10/rpg-lang/releases/download/v0.1.1/linux-aarch64.tar.gz)
- [Windows](https://github.com/Jomy10/rpg-lang/releases/download/v0.1.1/windows-x86_64.tar.gz)

If you have **HomeBrew** installed, you can also run:
```
brew tap rpgc && brew install rpgc
```

*Current version: v0.1.1*

### Minimum operating system:
- MacOS: 64-bit 10.7+
- Linux: 64-bit (kernel 4.2, glibc 2.17+)
- Windows: 64-bit Windows 7+

If you need a 32-bit binary, you can compile the cli manually by running `cargo build --release` inside of the rpg-cli folder . 
This will build the cli for your machine and put it in the target/release folder

## Tools
To make writing in this language easier, there is a VSCode extension for syntax highlighting. You can get it from the [VSCode Marketplace](hittps://marketplace.visualstudio.com/items?itemName=JonasEveraert.rpg-lang).

If you use a different IDE and want to add support for RPG, feel free to open a pull request!

## Contributing
I welcome all contributions. Feel free to open an issue about anything and optionally a pull request.

Also, if you want to, please take a look at the issues, some of them are improvements I want to make, others are bugs 
that need ot be fixed. I could use soe help with them.

## Using the CLI
To compile your rpg program, you can use the cli:

```bash
rpgc path/to/source.rpg
```

This command will create an executable file called `rpg` at your current location (`rpg.exe` on Windows). Make sure you don't have  a file called rpg there, as it will be overwritten.

To run your program, run `./rpg`. You can also combine these 2:

```bash
rpgc path/to/source.rpg && ./rpg
```

If something doesn't seem to work, you can always use the `-v` or `--verbose` flags to see if any errors occured. 
If they did, please open an issue as these kinds of errors are usually bugs in the compiler.

```bash
rpgc rpgc path/to/source.rpg -v
```

For more commands, use `rpgc help`.

If you have installed the cli using the manual downloads, you can run it using `./rpgc` or by moving it to your bin directory.

## Examples
You can find the examples in the [examples](examples) folder.

## Bugs
I have tested the compiler, but since this is still the first version, there might be some bugs, so feel free to open an issue.

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
Actors can attack each other, this subtracts the attack of the attacking actor from the health of the actor being attacked.

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
Characters can use [spell books](#spell-books) to cast spells. The spell name is
followed by `()` or `(param)`

#### God_speech
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

#### Un_zombify
Converts a zombie to a character.

```
james_brown uses spell_book casting un_zombify(zombie1)
```

#### Confuse 
When a character is confused, it will output its health - 1 when shouting or whispering.

```
steven uses spell_book casting confuse(other_char)
other_char shouts # Will output the other_char's health - 1 e.g. other_char = (1,2), so the output will be: 0
```

#### Create_potion
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

#### Shift
Shift swaps a character's health and attack.

```
char Ness = (2,1)
Ness uses spellbook casting shift()
Ness shouts
# Output:
# 1
```

## Questions
If you have any questions, feel free to ask them by opening an issue!

## Other links
- [Wiki page](https://esolangs.org/wiki/Rpg)

## License
The compiler and programming language are licensed under the [MIT License](LICENSE).
