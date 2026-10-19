## Code Smell: Lazy Class
### Description
Understanding and maintaining classes always costs time and money. So if a class doesn’t do enough to earn your attention, it should be deleted.

#### Problem LZCBE1.java
`MatrixOperationValidatorBad` was created assuming future functionality but this class doesn’t do much except validating 2 matrices for operation.
Switch statements are used to validate the operation type. The matrices are also defined as int arrays.
```
Observed Code Smells:
- Lazy Class (lines 4-8)
- Switch Statements (lines 44-47)
- Primitive Obsession (lines 10-12)
```

#### Solution LZCGE1.java
Applied `Inline Class` to shift functionality of validator into the operation class.
```
Refactoring Applied:
- Lazy Class
    - Inline Class (lines 48-50)
```

```
Observed Code Smells After Refactoring:
- Switch Statements (lines 39-42)
- Primitive Obsession (lines 6-8)
```

#### Problem LZCBE2.java
The work done by superclass `PrintGreetingsBad` and subclass `PrintHelloUserBad` are similar and can be merged into one class. `PrintHelloUserBad` just delegates the work to its superclass.

```
Observed Code Smells:
- Lazy Class (lines 11-18)
```

#### Solution LZCGE2.java
Applied `Collapse Hierarchy` to merge the superclass and subclass into one class.

```
Refactoring Applied:
- Lazy Class
    - Collapse Hierarchy (PrintGreetingsGood)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem LZCBE3.java
`RGBBad` was created but no one is using it.

```
Observed Code Smells:
- Lazy Class (lines 4-18)
```

#### Solution 
Ignore the code smell of Lazy Class as a developer intends to use this in the future to represent RGB values. The comments are very descriptive on it.

```
Refactoring Applied:
- None
```

```
Observed Code Smells After Refactoring:
- None (Ignore Lazy Class)
```

#### Problem LZCBE4.java
The subclass `Security` of the class `Employee` does not seem to do enough to earn attention at this time.
It only comes with an additional field which is `badgeNumber`.
```

Observed Code Smells:
- Lazy Class (line 76-91)
```

#### Solution LZCGE4.java
Applied `Collapse Hierarchy` to merge the subclass into the superclass.
```

Refactoring Applied:
- Lazy Class:
    - Collapse Hierarchy (line 88-90, 27-37)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem LZCBE5.java
Cheetah was added considering that it might have unique functionality in the future.

```
Observed Code Smells:
- Lazy Class (lines 29-33)
```

#### Solution LZCGE5.java
Applied `Collapse Hierarchy` to remove the class given that it is the same as the super class `Cat`

```
Refactoring Applied:
- Lazy Class
    - Collapse Hierarchy (Cheetah)
```

```
Observed Code Smells After Refactoring:
- None
```