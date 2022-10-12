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