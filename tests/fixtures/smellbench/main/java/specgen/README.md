## Code Smell: Speculative Generality 
### Description
There’s an unused class, method, field or parameter that was reserved for future use.

#### Problem SGBE1.java
An abstract class `AccomodationBad` was created with the hope of future use, but it was never used anywhere.
```
Observed Code Smells:
- Speculative Generality (lines 1-4)
```

#### Solution SGGE1.java
Deleted the unused abstract class by `Collapse Hierarchy`.

```
Refactoring Applied:
- Speculative Generality
    - Collapse Hierarchy (Deleted the unused abstract class AccomodationBad)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem SGBE2.java
The structure of different memberships are there for strategic design, but the memberships are not 
currently put into use.
```

Observed Code Smells:
- Speculative Generality (line 10-59)
```

#### Solution SGGE2.java
Simply delete the code to get the codebase organized.
```

Refactoring Applied:
- Remove Method:
    -  (The classes and methods no longer exist.)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem SGBE3.java
An abstract class `Shape` was created with the hope of future use, but it was never used anywhere.

```
Observed Code Smells:
- Speculative Generality (lines 1-5)
```

#### Solution SGGE3.java
Deleted the unused abstract class by `Collapse Hierarchy`.

```
Refactoring Applied:
- Speculative Generality
    - Collapse Hierarchy (Deleted the unused abstract class Shape)
```

```
Observed Code Smells After Refactoring:
- None
```