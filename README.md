## Clarifications

I participated in this [challenge](https://adventofcode.com/2022) programming in [Rust](https://www.rust-lang.org/), I've never worked with this language, but reading the documentation has always fascinated me. Below, you will also find some logic that I have implemented, absolutely improvable.

# π Advent of Code

## Day 01 / Part 1

In the first puzzle you are asked to locate the elfπ§ with the most calories π¬ carried. The list received as input is represented by integer numbers divided into lines indicating the calories transported. These lines are further divided by empty lines. When an empty line is encountered it will mean that the next set of calories is being carried π by a different elf.

### Flow
```Ascii
ββββββββββββββββ        ββββββββββββββββββββββ
βGet Input DataβββββββββΊβDefine index = 0    β
ββββββββββββββββ        β                    β
                   ββββββ€Define π§elf vect   β
                   β    ββββββββββββββββββββββ
                   βΌ
             ββββββββββββ
             βLOOP LINESβββββββββββββββββββββββ
             βββββββ¬βββββ                     β
                   β                          β
           βββββββββ΄ββββββββ                  β
    ββββββββ€ Line is Empty?ββββββββ           β
    βΌ      βββββββββββββββββ      βΌ           β
   NO                            YES          β
    β                             β           β
    β                             βΌ           β
    β                        βββββββββββ      β
    β                        βIndex++; β      β
    β                        ββββββ¬βββββ      β
    β                             β           β
    β                             β           β
    β     βββββββββββββββββββ     β           β
    ββββββΊβExist elf[index]?βββββββ           β
          βββββ¬βββββββββ¬βββββ                 β
              β        β                      β
    βββββYESβββ        ββββΊNO βββββ           β
    β                             β           β
    β                  ββββββββββββ΄ββββββββββ β
    β                  βPush 0 at elf[index]β β
    β                  ββββββββββββ¬ββββββββββ β
    β                             β           β
    β βββββββββββββββββββββββββββ β           β
    ββΊβAdd calories line at     βββ           β
      βelf[index]               β             β
      βββββββββββββ¬ββββββββββββββ             β
                  β                           β
            βββββββ΄βββββ                      β
            β END LOOP βββββββΊNOβββββββββββββββ
            βββββββ¬βββββ
                  β
                  βΌ
                 YES
                  β
                  β
                  βΌ
         ββββββββββββββββββββββββ
         βGet elf with max valueβ
         ββββββββββββββββββββββββ
```
## Day 01 / Part 2
The elfπ§ with the most caloriesπ¬ may have run out of stash, the challenge. in addition to the previous one, he asks to find the three elves who have the most calories.

### Flow

The flow is similar to part 1, only the result should be: take the three elvesπ§ that have the most caloriesπ¬ and add the result