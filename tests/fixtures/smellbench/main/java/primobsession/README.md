## Code Smell: Primitive Obsession
### Description
- Use of primitives instead of small objects for simple tasks (such as currency, ranges, special strings for phone numbers, etc.)

- Use of constants for coding information (such as a constant USER_ADMIN_ROLE = 1 for referring to users with administrator rights.)

- Use of string constants as field names for use in data arrays.

#### Problem POBE1.java
The class `HealthRecordBad` has a lot of primitives in it, including primitive arrays.

```
Observed Code Smells:
- Primitive Obsession (lines 4, 3-14)
```

#### Solution POGE1.java
Applied `Replace Data Value with Object` by grouping some primitives into a single objects. `Introduce Parameter Object` to replace multiple parameters with a single object. `Replace Array with Object
` was also applied for array based primitives.

```
Refactoring Applied:
- Primitive Obsession
    - Replace Data Value with Object (MedicalBioDataGood, PatientGood)
    - Introduce Parameter Objects (AddressGood, MedicalBioDataGood, PatientGood, MedicationHistory)
    - Replace Array with Object (MedicationHistory, MedicalHistory, AddressGood, NameGood)
```

```
Observed Code Smells After Refactoring:
- Data Class (NameGood, MedicalBioDataGood, MedicationHistory)
- Primitive Obsession (line 22)
```

#### Problem POBE2.java
The class `TeachingStaffBad` uses `empType` as a primitive to differentiate between different types of employees. Also, the `salary` is decided based on a switch statement on this type code.

```
Observed Code Smells:
- Primitive Obsession (lines 5, 26-34)
- Switch Statements (lines 26-34)
```

#### Solution POGE2.java
Applied `Replace Type Code with Subclasses` to replace the type code with subclasses. 

```
Refactoring Applied:
- Primitive Obsession
    - Replace Type Code with Subclasses (ProfessorGood, AssistantProfessorGood, AssociateProfessorGood)
- Switch Statements
    - Replace Type Code with Subclasses (ProfessorGood, AssistantProfessorGood, AssociateProfessorGood)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem POBE3.java
For the array of TutorialModals, it poses the issues of Primitive Obsession rather than using an Object
to encapsulate the information related to TutorialModals.
```
Observed Code Smells:
- Primitive Obsession (line 9, lines 15-19).
```

#### Solution POGE3.java
Applied `Replace Array with Object` to get rid of the array of type String.
A new class `TutorialModalManager` is created to deal with assign tutorials.

```
Refactoring Applied:
- Primitive Obsession
    - Replace Array with Object (line 6-45, line 50, lines 61-63).
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem POBE4.java

```
Observed Code Smells:
- Primitive Obsession (lines 2-5)
    - Too many primitive variables and a method that can be put into a separate class to represent itself.
```

#### Problem POBE5.java

```
Observed Code Smells:
- Primitive Obsession (line 2)
    - Use of primitive array to store information of logically grouped variables based on index
```

#### Solution POGE4.java
This solution applies for both POBE1 and POBE2
Applied `Replace Data Value with Object` treatment

```
Refactoring Applied:
- Primitive Obsession
    - Added `CardboardBox` class to move primitives into their own class
```

```
Observed Code Smells After Refactoring:
- None
```