## Clarifications

I participated in this [challenge](https://adventofcode.com/2022) programming in [Rust](https://www.rust-lang.org/), I've never worked with this language, but reading the documentation has always fascinated me. Below, you will also find some logic that I have implemented, absolutely improvable.

# 🎄 Advent of Code

## Day 01 / Part 1

In the first puzzle you are asked to locate the elf🧝 with the most calories 🍬 carried. The list received as input is represented by integer numbers divided into lines indicating the calories transported. These lines are further divided by empty lines. When an empty line is encountered it will mean that the next set of calories is being carried 🚚 by a different elf.

### Flow
```Ascii
┌──────────────┐        ┌────────────────────┐
│Get Input Data├───────►│Define index = 0    │
└──────────────┘        │                    │
                   ┌────┤Define 🧝elf vect   │
                   │    └────────────────────┘
                   ▼
             ┌──────────┐
             │LOOP LINES│◄────────────────────┐
             └─────┬────┘                     │
                   │                          │
           ┌───────┴───────┐                  │
    ┌──────┤ Line is Empty?├──────┐           │
    ▼      └───────────────┘      ▼           │
   NO                            YES          │
    │                             │           │
    │                             ▼           │
    │                        ┌─────────┐      │
    │                        │Index++; │      │
    │                        └────┬────┘      │
    │                             │           │
    │                             │           │
    │     ┌─────────────────┐     │           │
    └────►│Exist elf[index]?│◄────┘           │
          └───┬────────┬────┘                 │
              │        │                      │
    ┌────YES◄─┘        └──►NO ────┐           │
    │                             │           │
    │                  ┌──────────┴─────────┐ │
    │                  │Push 0 at elf[index]│ │
    │                  └──────────┬─────────┘ │
    │                             │           │
    │ ┌─────────────────────────┐ │           │
    └►│Add calories line at     │◄┘           │
      │elf[index]               │             │
      └───────────┬─────────────┘             │
                  │                           │
            ┌─────┴────┐                      │
            │ END LOOP ├─────►NO──────────────┘
            └─────┬────┘
                  │
                  ▼
                 YES
                  │
                  │
                  ▼
         ┌──────────────────────┐
         │Get elf with max value│
         └──────────────────────┘
```
## Day 01 / Part 2
The elf🧝 with the most calories🍬 may have run out of stash, the challenge. in addition to the previous one, he asks to find the three elves who have the most calories.

### Flow

The flow is similar to part 1, only the result should be: take the three elves🧝 that have the most calories🍬 and add the result