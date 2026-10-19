## Code Smell: Dead Code
### Description
A variable, parameter, field, method or class is no longer used (usually because it’s obsolete).

#### Problem DEDCBE1.java
Physics engine has dead variable `distance`, dead method `calculateDistance()` and dead parameter `gravity` in the constructor.
```
Observed Code Smells:
- Dead Code (lines 6, 8 and 15)
```

#### Solution DEDCGE1.java
`Deleted` unused code such as dead variable and dead method. Applied `Remove Parameter` on the unused parameter.

```
Refactoring Applied:
- Dead Code
    - Deleted unused code (distance and calculateDistance())
    - Remove Parameter (gravity)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem DEDCBE2.java
The method `calculate2018BlackFridayPrice` is no longer applicable.
Need to be refactored or deleted.
```

Observed Code Smells:
-  Dead Code (line 34-50)
```

#### Solution DEDCGE2.java
Simple delete the code.
```

Refactoring Applied:
- Dead Code:
    - Remove Method (No longer exist).
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem DEDCBE3.java
Shipment class has dead method `calculateDistance`  which has been 
superseeded by method `calculateDistanceFast` and thus also making
method `degToRad` dead.
```
Observed Code Smells:
- Dead Code (lines 9-14, 27-29)
```

#### Solution DEDCGE3.java
`Deleted` unused dead methods `calculateDistance` and `degToRad`.

```
Refactoring Applied:
- Dead Code
    - Deleted unused methods (calculateDistance and degToRad)
```

```
Observed Code Smells After Refactoring:
- None
```