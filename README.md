# 🗄️ McServer Manager
*This repository is very much at a work in progress*. Do not expect a build anytime soon

- [x] Mod Parsing
- [x] Plugin/Mod Backend Libraries
  - [x] Modrinth
  - [x] Hangar
  - [ ] Poggit
  - [ ] Curseforge
- [x] Maven Artifact Resolver
- [ ] Server backend architecture
- [ ] Aternos Clone

# What happened?
Currently, I am not satisfied on the state of this repository. Working on this project doesnt feel like it used to already.

Because of that, I will be performing a somewhat of a redo of this repository, now fully focusing on making
a server hosting platform. Here are the changes:
- `mpcli` will be deleted
- A possible merge of the `modrinth`, `hangar`, etc. backend crates might be
  considered. Either this or
- A dedicated organization for this project will be made and its libraries be split into their own separate repositories.
  - This approach has the massive downside of being unable to set up a Cargo Workspace in a reasonable way. I really need to unify as many dependencies as possible because I know I will be writing a lot of libraries to support this one program.

# A few caveats
> *if you will not be using this codebase as a developer, then there is no real need to read this*

<details>

## `denji`: The Minecraft Docker Image Manager & Builder
- *specifically on the **builder** side of things,*
  - `denji` has a weird quirk where the `software_version` for `denji::ServerSoftwareOptions` actually refers to the *artifact version*.
    - This quirk is not apparent for Neo/Forged servers, as the server installers *only install the server version specified in their artifact version*
    - For Fabric and Quilt servers, however, since they are installer-based JARs, they're artifact versions point to the *installer*'s artifact version, ***NOT THE SERVER'S***

</details>