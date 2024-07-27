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
  - [ ] Docker image manager/runner
  - [ ] Docker image builder
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
