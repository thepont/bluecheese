## Code Smell: Long Parameter List
### Description
More than three or four parameters for a method.

#### Problem LPLBE1.java
The constructor of PersonBad has 9 parameters as input. Also `accountNumber` is of primitive type.

```
Observed Code Smells:
- Long Parameter List (line 41)
- Primtive Obsession (line 2)
```

#### Solution LPLGE1.java
We applied `Preserve Whole Object` to make the parameters into a single object. 

```
Refactoring Applied:
- Long Parameter List
    - Preserve Whole Object (AddressGood address, BankAccountGood bankAccount)
```

```
Observed Code Smells After Refactoring:
- Primtive Obsession (line 2)
```

#### Problem LPLBE2.java
`calculateCartPrice()` method has 5 parameters. This is due to the method taking in different values from multiple different objects during its call.

```
Observed Code Smells:
- Long Parameter List (line 70)
```

#### Solution LPLGE2.java
Applied `Replace Parameter with Method Call`. The class `ShoppingCartGood` now takes in information on the store's pricing policy and the member's pricing policy. `calculateCartPrice()` now queries these objects directly internally for the required values.


```
Refactoring Applied:
- Long Parameter List
    - Replace Parameter with Method Call (shifted queries for values inside the method calculateCartPrice())
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem LPLBE3.java
`TriangleBad` takes in 6 parameters to assign as coordinates to the points of the triangle.

```
Observed Code Smells:
- Long Parameter List (line 18)
```

#### Solution LPLGE3.java
Applied `Introduce Parameter Object` to replace the parameters with parameter objects of `Coordinates2DGood`. This reduces the number of parameters in the constructor.

```
Refactoring Applied:
- Long Parameter List
    - Introduce Parameter Object (replaced parameters of constructor of TriangleGood with objects of Coordinates2DGood)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem LPLBE4.java
The logic of activatePromotion is there but instead of calling the relevant method to catch the logic, 
it passes new unnecessary parameters.
```
Observed Code Smells:
- Long Parameter List (lines 44-66)
```

#### Solution LPLGE4.java
Applied `Replace Parameter with Method Call` to get rid of the long parameter list but preserve the logics.

```
Refactoring Applied:
- Long Parameter List
    - Replace Parameter with Method Call (line 45-66)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem LPLBE5.java
The method calculateDistance has 4 parameters (2 of which are logically similar)

```
Observed Code Smells:
- Long Parameter List (line 4)
```

#### Solution LPLGE5.java
Applied 'Introduce Parameter Object' and added a LocationCoordinates class to hold the values with getters.

```
Refactoring Applied:
- Long Parameter List
    - Introduce Parameter Object (LocationCoordinates)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem LPLBE6.java
calculateDistance has 4 method parameters (all of which are simply values from another object)

```
Observed Code Smells:
- Long Parameter List (line 4, line 44)
```

#### Solution LPLGE6.java
Applied 'Preserve Whole Object' and passed the entire object 'boston' and 'newYork' instead of passing values derived from the object


```
Refactoring Applied:
- Long Parameter List
    - Preserve Whole Object
```

```
Observed Code Smells After Refactoring:
- None
```
