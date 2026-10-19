## Code Smell: Divergent Change
### Description
Divergent Change is when many changes are made to a single class.
You find yourself having to change many unrelated methods when you make changes to a class. 
For example, when adding a new product type you have to change the methods for finding, 
displaying, and ordering products.

#### Problem DVCHBE1.java
The printing for each operation seems to do similar things. So if any changes are made to the printing, it would have to be made in multiple places.
Representing matrices in form of int arrays constitutes primitive obsession.
```
Observed Code Smells:
- Divergent Change (lines 17, 26, 45)
- Primitive Obsession (lines 4-6)
```

#### Solution DVCHGE1.java
Applied `Extract Class` to extract out logging code to a separate logger class.

```
Refactoring Applied:
- Divergent Change
    - Extract Class (PrintLoggerGood)
```

```
Observed Code Smells After Refactoring:
- Primitive Obsession (lines 28-30)
```

#### Problem DVCHBE2.java
The services of Draw, Move and Resize are all inside one class which is LucyChartService.
If we add one more shape to the ShapeType Enum, the structure requires to change many spots altogether within
the LucyChart class, which constitutes a Divergent change.
```

Observed Code Smells:
-  Divergent Change (lines 75-158)
-  Switch Statements (lines 78-98, lines 103-122, lines 128-157)
```

#### Solution DVCHGE2.java
The services of Draw, Move and Resize should have their own classes and single responsibility.
```

Refactoring Applied:
- Divergent Change:
    - Extract class (lines 124-153, lines 155-184, lines 186-215)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem DVCHBE3.java
If we were to add a new type of connection, we would have to make changes to both the unrelated methods getDomain() and checkValidUrl()

```
Observed Code Smells:
- Divergent Change (lines 9-18, 20-27)
```

#### Solution DVCHGE3.java
Applied `Extract Class` to split the behavior into two classes. Both classes have the same behavior so, created interface Connector for inheritance

```
Refactoring Applied:
- Divergent Change
    - Extract Class (HTTPConnector, FTPConnector)
```

```
Observed Code Smells After Refactoring:
- None
```