# BOTC Fast Grimoire

Storyteller Assistance Tool to help you focus on making the fun decisions.  
***Currently Still In Progress: Can only run trouble brewing***

## Installation

Feel free to skip to step 3 if you already have rust installed. You can skip to step 4 if you already have trunk installed.

1. Install ```rustup```. You can view the documentation on how to do so [here](https://rust-lang.org/tools/install/)
2. Use rustup to install cargo

```shell
rustup toolchain install
```

3. Use cargo to install ```trunk```. Run the following command or see additional ways to do so [on their website](https://trunkrs.dev/#install)

```
cargo install trunk
```

4. Clone the Repository

```shell
git clone https://github.com/Altakie/BOTC-Fast-Grimoire.git
```

5. Install tailwindcss any way you wish and modify [Trunk.toml](./Trunk.toml) to point to your binary of tailwind. Don't change any of the arguments to the command.
6. Now you can just run

```shell
trunk serve
```

and you should be able to access the Storyteller Assistant at [localhost:8080](http://localhost:8080/)

Apologies for the long setup process. Currently the project is in progress and I have not yet packaged it so it can be easily distributed. I plan to eventually have it hosted as a website as well, so anyone can access it.

## Currently Implemented Roles

### Townsfolk

- Washerwoman
- Librarian
- Investigator
- Chef
- Empath
- Fortune Teller
- Undertaker
- Monk
- Ravenkeeper
- Virgin
- Slayer
- Soldier
- Mayor

### Outsiders

- Butler
- Drunk
- Recluse
- Saint (Currently in game but does nothing)

### Minions

- Poisoner
- Spy
- Baron
- Scarlet Woman (Currently placeholder implementation, can be used but not recommended)

### Demons

- Imp

## Future Features

- See [Requirements.md](./REQUIREMENTS.md) for planned features

## Known Bugs

I'm waiting to implement testing until I have a more stable API, so for now I will be manually finding bugs and logging them in [Bugs.md](./BUGS.md)
