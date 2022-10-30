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
    * [ ] Import = 2,
        * [x] Table
        * [x] Memory
        * [ ] Global
        * [ ] Function
    * [x] Function = 3,
    * [ ] Table = 4,
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

# Type section bytes:

* number of types of variables defined within the function
* which type (60 = function type)
* Number of input parameters leb128
* Input parameter types
* Number of output parameters
* Result types

# Function section bytes:

* number of functions (leb128)
* function indexes u8[]

# Export section bytes:

* number of exports
* length of export name leb128
* export name
* if more than 1 export -> 2 byte export description.

| Export Type | Export ID |
|-------------|-----------|
| func        | func id   |
| table       | table id  |
| memory      | mem id    |
| global      | global id |

# Import section type

* number of imports
* length of import module name (leb128)
* import module name
* length of import name name (leb128)
* import name name
* import description

| Import Type | Import ID |
|-------------|-----------|
| func        | func id   |
| table       | table id  |
| mem         | mem id    |
| global      | global id |

# build.sh

compiles all .wat files in the wat folder in .wasm fils in wasm folder. This is used
for testing purposes. 
