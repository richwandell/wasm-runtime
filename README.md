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
    * [x] Memory = 5,
    * [x] Global = 6,
    * [x] Export = 7,
    * [x] Start = 8,
    * [ ] Element = 9,
    * [ ] Code = 10,
        * [ ] test 0 page
        * [ ] test 1 page
        * [ ] test 2 page
        * [ ] test 3 page
        * [ ] test 4 page
        * [ ] test 5 page
        * [ ] test 6 page
        * [ ] test 7 page
        * [ ] test 8 page
        * [ ] test 9 page
        * [ ] test A page
        * [ ] test B page
        * [ ] test C page
        * [ ] test D page
        * [ ] test E page
        * [ ] test F page
    * [x] Data = 11,
    * [ ] DataCount = 12
* [ ] Create Enums for each type of section
* [ ] Create Enums for each instruction
    * [ ] Parse code section into instruction enums
* WASI
* [ ] [application abi](https://github.com/WebAssembly/wasi-io/blob/main/design/application-abi.md)
    * [ ] _start
    * [ ] _initialize
    * [ ] export linear memory
    * [ ] export __indirect_function_table
* [ ] [optional imports](https://github.com/WebAssembly/wasi-io/blob/main/design/optional-imports.md)

# build.sh

compiles all .wat files in the wat folder in .wasm fils in wasm folder. This is used
for testing purposes. 
