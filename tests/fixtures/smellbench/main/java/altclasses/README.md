## Code Smell: Alternative Classes With Different Interfaces
### Description
Two classes perform identical functions but have different method names.

#### Problem ACDIBE1.java
`LionBad` and `RabbitBad` do similar stuff although they have different method names.
```
Observed Code Smells:
- Alternative Classes With Different Interfaces (lines 1-15 and 17-31)
```

#### Solution ACDIGE1.java
Made a common interface. Applied `Rename Methods`. Did `Extract Superclass` to move the common functionality to the abstract class and made the remaining classes as subclasses.
```
Refactoring Applied:
- Alternative Classes With Different Interfaces
    - Rename Method (animalVoice and animalInfo)
    - Extract Superclass (AbstractAnimal)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem ACDIBE2.java
In the bad example, the developer put too much emphasis on the subject that makes the action,
for example, a bishop can `getBishopRow()`, `getBishopCol()`, `getBishopColor()`, `bishopCanMove()`, `bishopCanKil()`.
In the scenario of the international chess, all the chess pieces have these functionalities.
It doesn't matter who makes the action, just be careful with the rule for each of them respectively.
Therefore, have these chess pieces classes written separately is not ideal, even the names of the methods are different
(which means the subject who makes that action differs), the functionality ends up being the same.
Each of the chess piece can get its locations and color, can kill another chess piece and can move within the bounds.
```
Observed Code Smells:
- Alternative Classes With Different Interfaces (lines 11-29, lines 41-69,
lines 78-93, lines 107-153, lines 164-179, lines 192-238, lines 253-273, lines 317-405,
lines 422-437, lines 449-495, lines 503-518, lines 530-576)
```

#### Solution ACDIBE2.java
Applied `Extract superclass` to summarize the getter methods and constructors.
Applied `Rename Method` to get rid of the subject, e.g., `bishopCanKill()` -> `canKill()`.

```
Refactoring Applied:
- Alternative Classes With Different Interfaces
    - Extract superclass (lines 18-55, lines 58-64, lines 88-94, lines 124-130, lines 163-174, lines 235-241, 
     lines 265-271).
    - Rename Method (lines 36-54).
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem ACDIBE3.java
There are two different interfaces each of which have their respective implementations all of which are identical but only in method names and field names.
```
Observed Code Smells:
- Alternative Classes With Different Interfaces (lines 12-20 and 65-107)
```

#### Solution ACDIGE3.java
Removed one of the similar interface and its class.
```
Refactoring Applied:
- Alternative Classes With Different Interfaces
    - Delete alternate interface and its implementation
```

```
Observed Code Smells After Refactoring:
- None
```