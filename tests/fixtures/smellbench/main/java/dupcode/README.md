## Code Smell: Duplicate Code
### Description
Two code fragments look almost identical.
#### Problem DUPCBE1.java
The print statements are duplicated across all methods. Also `accountNumber` is represented by a primitive while it should have some validation or a different type
```
Observed Code Smells:
- Duplicate Code (lines 14-16, 21-23, 29-31)
- Primitive Obsession (line 4)
```

#### Solution DUPCGE1.java
Applied `Extract Method` as it’s a duplication in the same class.

```
Refactoring Applied:
- Duplicate Code
    - Extract Method (printAccountDetails)
```

```
Observed Code Smells After Refactoring:
- Primitive Obsession (line 4)
```

#### Problem DUPCBE2.java
Usually, the duplicated part can be extracted as a method.
It may be just the code are produced by 2 different developers and someone has to refactor
to get rid of the duplication.
```

Observed Code Smells:
-  Duplicated Code (lines 26-28, lines 41-43)
```

#### Solution DUPCGE2.java
Identify the duplicated code and extract them as a private helper method.
```

Refactoring Applied:
- Duplicated Code:
    - Extract method (lines 42-49)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem DUPCBE3.java
Functions Add() and Sum() are functionally identical.

```
Observed Code Smells:
- Duplicate Code (lines 10-12, 26-29 )
```

#### Solution DUPCGE3.java
Applied `Extract Method` as it’s a duplication in the same class.

```
Refactoring Applied:
- Duplicate Code
    - Extract Method (Sum)
```

```
Observed Code Smells After Refactoring:
- None
```