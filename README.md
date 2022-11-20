JFF i'm working on a wasm runtime. Goal of this project is just for fun. Learning and exploring the wasm format and
abstract vm.
Ill slowing add to this repo as I progress on this journey. This readme will be a living document
where I can add notes and update my roadmap. I'm hoping I can use this project for a blog post or two and
maybe a few videos and talks on how wasm works.

![Build Status](https://github.com/richwandell/wasm-runtime/actions/workflows/rust.yml/badge.svg)

# Roadmap

-------

* [ ] Read all sections
    * [ ] Custom = 0,
    * [x] Type = 1,
    * [x] Import = 2,
    * [x] Function = 3,
    * [x] Table = 4,
    * [ ] Memory = 5,
    * [ ] Global = 6,
    * [x] Export = 7, 
    * [ ] Start = 8,
    * [ ] Element = 9,
    * [ ] Code = 10,
    * [ ] Data = 11,
    * [ ] DataCount = 12
* [ ] Create Enums for each type of section
* [ ] Create Enums for each instruction
    * [ ] Parse code section into instruction enums


# build.sh

compiles all .wat files in the wat folder in .wasm fils in wasm folder. This is used
for testing purposes. 
