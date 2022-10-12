# Type section bytes:

* 0 - number of types of variables defined within the function
* 1 - which type (60 = function type)
* 2 - Number of input parameters leb128
* 3 -> 3+@2 - Input parameter types
* 3+@2 - Number of output parameters
* 3+@2 -> @(3+@2) - Result types

# Function section bytes:

* 0 -

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

# build.sh

compiles all .wat files in the wat folder in .wasm fils in wasm folder. This is used
for testing purposes. 