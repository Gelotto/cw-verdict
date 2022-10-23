# Gelotto Ballot

A CosmWasm smart contract that implements a game of voting. Players can purchase
votes on one or more possible outcomes of an event, as described by the prompt
of a trial. Each trial is presented in the form of a multiple-choice question.
Upon a specified date and time, one or more trusted parties run a script
off-chain, which consults an outside authority, like a weather or sporting event
API, and returns an integer code that maps to one of the trial's choices. Both
the output of the script and its execution logs are sent back and stored in the
contract. The source code for the script is stored transparently on-chain.

## Execute API

### Vote

#### Arguments

- weight: number (u32)
- choice: number (u32)

Wallets can cast votes as long as a trial is "active". Each vote carries a
"weight". The cost of casting a vote is equal to the weight of the vote
multiplied by a unit price, defined by `Ballot.price`. A wallet can cast votes
multiple times, either on the same choice or different. For example, they can
place a weight of 5 on choice "A" and a weight of 1 on choice "B".

### Decide

#### Arguments

- logs: output generated by decision script
- choice: winning choice index output by script

Only members of the "jury" can execute the `decide` method, in which they upload
the result of running the trial's decision script. The last "juror" to execute
this method puts the contract into the `decided` state in which, after a defined
period of time, winners can claim their prize. If any member of the jury submits
a winning choice that differs from the winning choices uploaded by other members
of the jury so far, the contract goes to the `hung` state, and each wallet may
call the `claim` method to receive a complete refund.

### Claim

Any player who voted can call this method under several conditions. If the game
was canceled or the jury was hung, a player can claim a complete refund. On the
other hand, if the player who voted has won, they can claim their portion of the
prize. The size of their portion depends on the weight of their winning vote.

### Cancel

The owner of the contract can call this method to cancel the game so long as the
contract is either in the `active` or `deciding` state. Once canceled, players
can claim a refund through the `claim` method.
