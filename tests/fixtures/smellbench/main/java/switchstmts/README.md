## Code Smell: Switch Statements
### Description
You have a complex `switch` operator or sequence of `if` statements.


#### Problem SWSBE1.java
In `ShapeBad`, `calculateArea()` depends on the type of shape to decide which calculation to perform and how to access dimensions.
`shape` should have been an enum instead of a string.
```
Observed Code Smells:
- Switch Statements (lines 8-19)
- Primitive Obsession (line 2)
```

#### Solution SWSGE1.java
Applied `Replace Conditional with Polymorphism` to create individual classes implementing the `calculateArea()` method.

```
Refactoring Applied:
- Switch Statements
    - Replace Conditional with Polymorphism (CircleGood, RectangleGood, SquareGood, TriangleGood)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem SWSBE2.java
The clas `UserRouteBad` has a switch statement to decide the route based on the type code `routeType`.
```
Observed Code Smells:
- Switch Statements (lines 13-25)
```

#### Solution SWSGE2.java
Applied `Replace Type Code with Strategy` to replace the type code with different strategies to calculate effects of choosing different routes.

```
Refactoring Applied:
- Switch Statements
    - Replace Type Code with Strategy (CarRouteStrategy, BikeRouteStrategy, WalkRouteStrategy, BusRouteStrategy)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem SWSBE3.java
The method `performTest` currently contains a long switch statements based on the sample's test type.
The switch statement carries the responsibility of both deciding which test to go for and the concrete
implementations of that particular test. Therefore, single responsibility is not demonstrated here.
And also, the switch statement is too long.
```
Observed Code Smells:
- Switch Statements (line 39-100)
```

#### Solution SWSGE3.java
Applied `Extract Method` and `Move Method` to get rid of the long switch statements.
Set up an interface called `TestExecutor` and have different tests as different classes to implement this interface.
```
Refactoring Applied:
- Switch Statements:
    - Extract Method and Move Method (line 68-149)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem SWSBE4.java
In CommandParser there is a big switch case that calls methods based on the command given to it.
```
Observed Code Smells:
- Switch Statements (lines 33-53)
```

#### Solution SWSGE4.java
Applied Replace Type Code With Strategy to create a new class that encapsulates each possible command which can provide the command name and execute the action of the command.

```
Refactoring Applied:
- Switch Statements
    - Replace Type Code With Strategy (Action, AttackAction, ...)
```

```
Observed Code Smells After Refactoring:
- None
```