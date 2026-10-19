## Code Smell: Shotgun Surgery
### Description
Shotgun Surgery refers to when a single change is made to multiple classes simultaneously.

#### Problem SSBE1.java
Both `MathClassBad` and `EnglishClassBad` have private methods which seem to access the method of `SubjectsGradingBad` a lot and have similar methods. Changing grading scheme in one class might need to change the grading scheme in another.
Setting the grades is also repeated in both classes. Also `subjectGrad` could have been an enum.
Also both the classes have a very similar interface with different method names. So they can be Alternative Classes with Different Interfaces.
```
Observed Code Smells:
- Shotgun Surgery (lines 42-54 and 72-84)
- Feature Envy (lines 42-54 and 72-84)
- Duplicate Code (lines 43-52 and 73-82)
- Primitive Obsession (line 2)
- Alternative Classes with Different Interfaces (line 27-55 & 57-85)
```

#### Solution SSGE1.java
Applied `Move Method` to move the common logic to SubjectGradeGood. This reduces duplication and ensures that the logic for grading remains in the class it is meant to be in.
```
Refactoring Applied:
- Shotgun Surgery
    - Move Method (setGradeBasedOnMarks)
```

```
Observed Code Smells After Refactoring:
- Primitive Obsession (line 2)
- Alternative Classes with Different Interfaces (line 42-56 & 58-72)
```

#### Problem SSBE2.java
Imagine this system consist of many types of sensors (now 3), and each of the sensors has a battery inside of it. 
But essentially, batteries and sensors are separate entities. So it makes no sense, when we change something about the  
battery, we should go into every sensor classes to have that done. It leads to Shotgun issue.
```
Observed Code Smells:
- Shotgun Surgery (line 26, lines 28-29, line 38, line 42, )
```

#### Solution SSGE2.java
Applied `Move Method` and `Move Field` to the SensorData class to have the constants and the check logic managed
in a centralized data system.
```

Refactoring Applied:
- Shotgun Surgery 
  -  Move Field (lines 129-131)
  -  Move Method (lines 146-160)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem SSBE3.java
Both `CheckProcessor` class and `DirectDepositProcessor` classes print information about the `BankAccount` class. If there is any change in the way print is to be handled or another field is added to `BankAccount` class, change will need to be made to the other two classes as well.

```
Observed Code Smells:
- Shotgun Surgery (lines 59,94-95)
- Feature Envy (lines 59, 94-95)
```

#### Solution SSGE3.java
Applied `Move Method` to move the printing logic to `BankAccount` class itself which also removes `Feature Envy` and now any change made to printing logic or any new fields that need to be printed (ex: balance) can be made in one class.
```
Refactoring Applied:
- Shotgun Surgery
    - Move Method (printAccountDetails)
```

```
Observed Code Smells After Refactoring:
- None
```