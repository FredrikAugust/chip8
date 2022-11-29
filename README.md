# chip8

en implementasjon av chip8. varmer opp til gb

mange ting som kunne blitt gjort annerledes. eksempelvis

- map om opcodes til enums med verdier og ha egne funksjoner for å håndtere de
  - dette ville gjort at ikke all logikken lå inne i én `impl Cpu`
- skrevet tester for å sjekke at funksjoner gjør som de skal
  - burde ha én test per opcode for å se om den i isolasjon gjør det den skal. da kan du sammenlikne minne direkte
- finn en superenkel ROM og ha som mål å få den til å kjøre grønt
