## Code Smell: Refused Bequest
### Description
If a subclass uses only some of the methods and properties inherited from its parents, the hierarchy is off-kilter. The unneeded methods may simply go unused or be redefined and give off exceptions.

#### Problem RBBE1.java
`MobileBad` inherits `switchOnOff()` and `printDetails()` from `ComputerBad`, but it also ends up inheriting `clickedKeyboard()` and `moveMouse()` which it does not need.

```
Observed Code Smells:
- Refused Bequest (lines 62 and 66)
```

#### Solution RBGE1.java
Applied `Replace Inheritance with Delegation` to help `MobileGood` only maintain methods it needs.


```
Refactoring Applied:
- Refused Bequest
    - Replace Inheritance with Delegation (computer field in MobileGood)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem RBBE2.java
The interface `NoteTakerBad` enforces sharing notes as pdf which cannot be supported by `NotebookNotesBad`.

```
Observed Code Smells:
- Refused Bequest (line 25-27)
```

#### Solution RBGE2.java
Applied `Extract Superclass` to extract out the common methods to `NoteTakerGood` and got rid of unneeded methods from `NotebookNotesBad`.

```
Refactoring Applied:
- Refused Bequest
    - Extract Superclass (NoteTakerGood)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem RBBE3.java
The class `LegoPart` that related to toys,
intends to use the dimension getter method only from class `BuildingBrick` for construction,
Some more fields and methods in class `BuildingBrick` are not applicable to class `LegoPart`.
The inheritance relationship is embarrassing.
```

Observed Code Smells:
- Refused Bequest (line 44-108)
```

#### Solution RBGE3.java
Applied `Extract Superclass` method to consolidate the common dimension methods in to a 
common superclass `Block` and then have class `BuildingBlock` and class `LegoPart` to extends it with 
their own features.
```

Refactoring Applied:
- Refused Bequest:
    - Extract Superclass (line 12-45)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem RBBE4.java
Car and Bicycle both implement the interface Vehicle since they share common methods and logical attributes. However some of the methods of the interface only apply to the Car and not Bicycle and so the Bicycle is not able to fulfill some duties of Vehicle correctly since they do not apply to it.

```
Observed Code Smells:
- Refused Bequest (lines 93-101)
```

#### Solution RBGE4.java
Applied Extract Superclass and removed the non-common functions from the interface Vehicle. Created a new interface that extends the old interface.


```
Refactoring Applied:
- Refused Bequest
    - Extract Superclass(refuel() and replaceEngine() from Vehicle interface)
```

```
Observed Code Smells After Refactoring:
- None
```