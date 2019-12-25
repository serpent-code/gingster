# gingster
Gin Rummy playing engine. Written in Rust.

Gingster is a CLI gin rummy playing program. It's not a complete software suit to play gin rummy against the computer, rather it's a gin player only. It prompts the user to enter a starting hand and then asks what's happening in the actual game outside its control like what card the opponent dropped and so on. Then it would say things like pickup 7S or drop KC, etc and it expects the player does as it says in the actual game.

All card inputs have the form 1 character rank followed immediately by 1 character suit, case insesitive.

ranks:

Ace => A
2 ..= 9 => number itself
10 => T
Jack => J
Queen => Q
King => K

Suits:

S: Spades, C: Clubs, H: Hearts, D: Diamonds

This is one example of acceptable input when asked for the initial hand:

`as 2s 3s 4s 9c tc JC js qs ks`

At any point entering "end" in input it makes the melds it can, counts its deadwood and exits.

Currently it has no idea about a 100 point game score as a collection of hands either. It plays one hand and exits, either by knocking or typing "end".

It builds with the latest stable Rust and doesn't have any dependencies except Rust's standard library.
