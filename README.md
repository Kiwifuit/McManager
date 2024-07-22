# 🗄️ McServer Manager
*This repository is very much at a work in progress*. Do not expect a build anytime soon

- [x] Mod parsing
  - [x] Modrinth Support
  - [x] Forge Support
  - [x] Plugin support
- [ ] Plugin/Mod Backend libraries
  - [x] Modrinth backend library
  - [ ]  ~~Curseforge backend library~~ **Axed until further notice**
  - [x] Hangar backend library
  - [ ] BukkitDev backend library
    - This will be one hell of a webcrawling adventure :)
- [x]  Modpack parser
  - [x] Modpack packer?
    - Like a program that packs modpacks in a way that the aternos clone can parse
    - [ ] :hammer_and_wrench: General support (Un/Install & Info)
- [ ] Aternos clone
  - [ ] Basic functionality
    - [ ] Server downloading
    - [ ] Mod downloading
    - [ ] Interfaces (settings/blacklist/players/etc.)
  - [ ] World switching
  - [ ] Modpack loading
    - [ ] From Curseforge
    - [ ] From Modrinth
    - [ ] From File (Forge/Modrinth)

> [!NOTE]
> 🛠️ indicates the component currently worked on

*As you can see, this project is **FAR** from done, nor do I have any hope of finishing it, but I will do my best*

I will not follow this roadmap in order. I will prioritize Modrinth support over Curseforge. Hangar, BukkitDev and the like will be worked on ***dead last*** for one of the following reasons:
- Shit/Underdeveloped API
- No API. Gotta crawl the things

# Customization
I'm not gonna provide builds for *every single possible combination of features*, that takes too much time, instead read the guides for customizing the binaries for:
- [`mpcli`](#mpcli-minecraft-modpack-imexporter)
- [Upcoming] `mcs`

Doing this obviously requires the source code and build tools, so make sure the following are installed:
- Git
- Cargo/Rust

> [!IMPORTANT]
> Going further than this part assumes you have:
> - cloned this repository, and
> - are `cd`'ed into said repo

## `mpcli`, Minecraft Modpack Im/Exporter
The features for this binary include:
- `modrinth` [Modrinth](https://modrinth.com/modpacks) modpack parse/install. **Enabled by default**
- `forge` [Curseforge](https://www.curseforge.com/minecraft/search?page=1&pageSize=20&sortBy=relevancy&class=modpacks) modpack parse/install
- `packing` adds the `export` subcommand, which allows you to...well...export currently installed modpacks. **Enabled by default**

Build with *only* modrinth and `export` command support:
```
cargo build --release --no-default-features --features modrinth,packing
```