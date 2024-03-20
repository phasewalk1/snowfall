# Snowfall

```text
    ...        *                        *       *
      ...   *         * ..   ...                        *
 *      ...        *           *            *
          ...               ...                          *
            ..                            *
    *        ..        *                       *
           __##____              *                      *
  *    *  /  ##  ****                   *
         /        ****               *         *  X   *
   *    /        ******     *                    XXX      *
       /___________*****          *             XXXXX
        |            ***               *       XXXXXXX   X
    *   | ___        |                    *   XXXXXXXX  XXX
  *     | | |   ___  | *       *             XXXXXXXXXXXXXXX
        | |_|   | |  ****             *           X   XXXXXXX
    *********** | | *******      *                X      X
************************************************************
```



**_Snowfall_** is a novel anonymous polling protocol that leverages zk-SNARKs to provide private, verifiable voting. It's written in Rust, and makes extensive use of the [arkworks](https://github.com/arkworks-rs) libraries. Many thanks to the amazing contributors who have built a delightful suite of crates for algebraic coding.

> [!WARNING]
> This is a student project. I'm trying to learn more about cryptography (and more specifically, ZK) engineering
> by working on real protocols. This is not yet production-grade, _nor has it been audited_. I would say "use at your
> own risk", but in fact, I'd advise (as of now) to not use this at all!

# Overview

Snowfall operates over the [`BLS12-381`]() curve.

## Registration

1. The voter generates a random scalar $\gamma$ and computes a [Pedersen commitment]() `comm` to their public key $pk$.
2. Voter sends `comm` to the voting authority (some smart contract).
3. Authority adds `comm` to the Merkle tree and returns the updated root.
4. Voter creates a `Registration` proof, proving knowledge of $\gamma$ and inclusion of `comm` in the Merkle tree.
5. The `Registration` circuit outputs a `nullifier`, which is a hash of `comm` and `gamma`.
6. Voter sends the `Registration` proof, the `nullifier`, and a [Boneh-Boyen signature]() (proving private key ownership) to the authority.
7. Authority verifies the proof and the signature, records the `nullifier`, and marks the voter as registered.


## Voting

1. Voter creates a Pedersen commitment to their vote.
2. Voter constructs a `Cast` proof, proving that they are a registered voter (using the `Registration` proof) and that the `nullifier` hasn't been used before.
3. The `Cast` circuit outputs the vote commitment.
4. Voter sends the `Cast` proof, the `nullifier`, and the vote commitment to the authority.
5. Authority verifies the proof, checks that the `nullifier` hasn't been used before, and if so, records the `nullifier` and adds the vote commitment to the list of cast votes.

## Tallying

1. After the voting period ends, the `Tally` circuit [homomorphically]() sums the vote commitments to get a commitment to the total tally.
2. Multiple authorities cooperate to open the tally commitment and reveal the final vote count.

# Spec

## Circuits

### `Registration`

The _registration_ circuit proves that the voter knows the `gamma` used to create `comm` and that `comm` exists in the Merkle tree of eligible voters.

It's **public inputs** are:

- `root`: The root of the Merkle tree
- `comm`: The Pedersen commitment to the voter's public key

It's **private inputs** (witnesses) are:

- `pk`: The voter's public key
- `gamma`: The random scalar used to create the commitment
- `path`: The Merkle path from the `comm` to the `root`

The circuit checks that:

1. `comm` is a valid Pedersen commitment to `pk` with randomness `gamma`
2. `path` is a valid Merkle path for `comm` and `root`.

It outputs:

- `nullifier`: The hash of `comm` and `gamma`.


### `Cast`

The _cast_ circuit proves that the voter is registered (by including the `Registration` proof) and that they haven't voted before (by checking the `nullifier`).

Its **public inputs** are:

- `root`: The root of the Merkle tree at time of registration
- `comm`: The Pedersen commitment to the voter's public key
- `vote_comm`: The Pedersen commitment to the voter's vote
- `nullifier`: The nullifier output by the `Registration` circuit
- `sig`: The Boneh-Boyen signature, proving ownership of the private key associated with the voter's public key

It's **private inputs** (witnesses) are:

- `vote`: The voter's actual vote
- `registration_proof`: The `Registration` proof
- `vote_gamma`: The random scalar used to create `vote_comm`

The circuit checks that:

1. `registration_proof` is a valid `Registration` proof for `comm` and `root`
2. `vote_comm` is a valid Pedersen commitment to `vote` with randomness `vote_gamma`
3. `sig` is a valid Boneh-Boyen signature for `comm` under the public key `pk` (contained in `registration_proof`)

### `Tally`

The _tally_ circuit homomorphically sums the vote commitments to get a commitment to the total tally.

Its **public inputs** are:

- `vote_comms`: The list of vote commitments
- `tally_comm`: The commitment to the total tally

Its **private inputs** (witness) are:

- `tally`: The actual vote tally

The circuit checks that:

1. `tally_comm` is the homomorphic sum of `vote_comms`.
2. `tally` is the opening of `tally_comm`.

### Security Considerations

- The `nullifier` is generated deterministically from the `comm` and the commitment randomness `gamma` to prevent double-voting. 
- In a production deployment, the Groth-16 setup procedure must be performed securely, ideally through a multi-party ceremony, to ensure the integrity of the parameters (something like Powers of Tau).
