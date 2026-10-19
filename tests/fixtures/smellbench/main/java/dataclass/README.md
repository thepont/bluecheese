## Code Smell: Data Class
### Description
A data class refers to a class that contains only fields and crude methods for accessing them (getters and setters). These are simply containers for data used by other classes. These classes don’t contain any additional functionality and can’t independently operate on the data that they own.
#### Problem DCLSBE1.java
`RectangleBad` only has setters and getters while the computation is done on the outside.
```
Observed Code Smells:
- Data Class (lines 1-17)
```

#### Solution DCLSGE1.java
Applied `Remove Setting Method`, and `Hide Method` for getters. Applied `Move Method` on the calculation of perimeter and area.
```
Refactoring Applied:
- Data Class
    - Remove Setting Method and Hide Method (Removed the getters for data)
    - Move Method (getPerimeter, getArea)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem DCLSBE2.java
There are 2 classes: Plane and PartStudio.
The misplacement of the method `calculateVolume` in PartStudio makes Plane a Data Class.
The method `calculateVolume` should be in class Plane.

```
Observed Code Smells:
-  Data Class (line 38-51).
```

#### Solution DCLSGE2.java
Apply `Move Method`, moving the method back to class Plane would help to get rid of the code smell.

```
Refactoring Applied:
- Data Class:
    - Move Method (line 20-29).
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem DCLSBE3.java
`Task` has only public fields and no getters or setters (encapsulation)
```
Observed Code Smells:
- Data Class (lines 4-8)
```

#### Solution DCLSGE3.java
Applied `Encapsulate Field` for name, description and completed fields. Add applied `Extract method` to create the `markAsCompleted` method in the Task class.
```
Refactoring Applied:
- Data Class
    - Encapsulate Field (name, description, completed)
    - Extract Method(markAsCompleted)
```

```
Observed Code Smells After Refactoring:
- None
```