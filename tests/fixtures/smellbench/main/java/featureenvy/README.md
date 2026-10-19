## Code Smell: Feature Envy
### Description
A method accesses the data of another object more than its own data.

#### Problem FEBE1.java
The data of `Coordinates2DBad` are accessed continuously. Also, the class does not have any other functionality except getters.
```
Observed Code Smells:
- Feature Envy (lines 40-41 and 47)
- Data Class (lines 1-12)
```

#### Solution FEGE1.java
Used `Move Method` to move calculation of distance to the coordinate class. `Extract Method` to extract out the data access and moved it to `Coordinates2DGood` class.

```
Refactoring Applied:
- Feature Envy
    - Move Method (calculateDistance)
    - Extract Method with Move method (sameX, sameY)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem FEBE2.java
The method `generateUsageReport` in class `SignalProcessor` currently generate report by accessing data from 
methods in class `RemoteController`, which makes a `Feature Envy` code smell.
We can move the functionality of generating report back to the class `RemoteController` and remove the 
related getter methods in class `RemoteController`.
```

Observed Code Smells:
-  Feature Envy (line 114-137)
```

#### Solution FEGE2.java
Applied `Move Method` to move the envied feature back to the class from which the data comes from.
```

Refactoring Applied:
- Feature Envy:
    - Move Method (line 50-77)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem FEBE3.java
The data of `Record` class, specifically genre, name and duration is accessed far more by the class `RecordPlayer` than `RecordPlayer` is accessing its own data

```
Observed Code Smells:
- Feature Envy (lines 57-61, 63-66)
```

#### Solution FEGE3.java
Used `Move Method` to move getFormattedDuration() and getCurrentRecordDetails() to the `Record` class as it is better suited.

```
Refactoring Applied:
- Feature Envy
    - Move Method (getFormattedDuration, getCurrentRecordDetails)
```

```
Observed Code Smells After Refactoring:
- None
```