## Code Smell: Parallel Inheritance Hierarchies
### Description
Whenever you create a subclass for a class, you find yourself needing to create a subclass for another class.

#### Problem PIHBE1.java
Implementing `CarBad` leads to implementing `CarFactoryBad` and implementing `BikeBad` leads to implementing `BikeFactoryBad`. Similarly, for other vehicle types we will have to keep on adding an extra class for every type created.
`engineType` could have been defined using an enum instead of a string.
```
Observed Code Smells:
- Parallel Inheritance Hierarchies (lines 42-104)
- Primitive Obsession (line 14)
```

#### Solution PIHGE1.java
Used `Move Method` and `Move Field` to move `Factory` functionalities into `Vehicle`. This also helped remove the factory hierarchy completely.

```
Refactoring Applied:
- Parallel Inheritance Hierarchies
    - Move Method and Move Field (All factory methods moved to Vehicle)
```

```
Observed Code Smells After Refactoring:
- Primitive Obsession (line 9)
```

#### Problem PIHBE2.java
Suppose we have 2 operating systems in the Platform Enum, which are iOS and Android.
They both extends the same set of abstract classes(super classes), but now for the same super classes, 
a newly-added child class for iOS must lead to a newly-added child class for Android. 
This constitutes a Parallel Inheritance Hierarchies. 
```

Observed Code Smells:
- Parallel Inheritance Hierarchies (lines 28-40, lines 46-58, lines 65-87)
```

#### Solution PIHGE2.java
Instead of having separate classes for different operating systems to implement the same contract,
have a operatingSystem to extend the super class which defines the functions and use EnumMap inside that 
operatingSystem class to switch among different operating systems.
```

Refactoring Applied:
- Parallel Inheritance Hierarchies:
   - Move Method and Move Field (lines 32-57, lines 63-88, lines 95-115)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem PIHBE3.java
There is a parallel heirarchy between the Weapon interface and Enemy interface where, each time a new enemy is made (for eg. Werewolf), a new weapon needs to be made to complement it (WerewolfWeapon).
```
Observed Code Smells:
- Parallel Inheritance Hierarchies (lines 66-118)
```

#### Solution PIHGE3.java
Removed hierarchy in `VampireWeapon` and `GhoulWeapon` by replacing them with a `BasicWeapon` and moved their fields and methods to the new `BasicWeapon` class using `Move Method` and `Move Field`.

```
Refactoring Applied:
- Parallel Inheritance Hierarchies
    - Move Method and Move Field (BasicWeapon)
```

```
Observed Code Smells After Refactoring:
- None
```