# Type section bytes:
* 0 - number of types of variables defined within the function
* 1 - which type (60 = function type)
* 2 - Number of input parameters
* 3 -> 3+@2 - Input parameter types
* 3+@2 - Number of output parameters
* 3+@2 -> @(3+@2) - Result types

# Function section bytes:
* 0 - 