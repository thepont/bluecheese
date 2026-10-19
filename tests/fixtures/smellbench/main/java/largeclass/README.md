## Code Smell: Large Class
### Description
A class contains many fields/methods/lines of code.

#### Problem LCBE1.java
In this file, we can see PersonBad contains 9 fields and 5 methods. A single class seems to be responsible for a person's information, address information and bank account information.
The 9 variables defined are also using primitive data types which can lead to primitive obsession.

```
Observed Code Smells:
- Large Class (line 3-52)
- Primitive Obsession (line 4-12)
```

#### Solution LCGE1.java
In this file, we have applied `Extract Class` to refactor `AddressGood` and `BankAccount` into their own classes to maintain their own responsibility. `Extract Interface` has been applied to maintain the methods available to be used by any client for any class. \
`Replace Data Value wih Object` is a special case of `Extract Class` where we replace a data value with an object. Address related fields are put in `AddressGood` class and Bank Account related fields are put in `BankAccountGood` class.

```
Refactoring Applied:
- Large Class
    - Extract Class (AddressGood, BankAccountGood)
    - Extract Interface (IAddress, IBankAccount, IPerson)
- Primitive Obsession
    - Replace Data Value with Object (AddressGood, BankAccountGood)
```

```
Observed Code Smells After Refactoring:
- Primitive Obsession (line 44, 46)
```

#### Problem LCBE2.java
In this file we can see that `PersonBad` class also contains the responsibility for being a student. `studentId` is also of primitive type.

```
Observed Code Smells:
- Large Class (line 5-25)
- Primitive Obsession (line 7)
```

#### Solution LCGE2.java
In this file, we have applied `Extract Subclass` to abstract out the student relation. 

```
Refactoring Applied:
- Large Class
    - Extract Subclass (StudentGood)
```

```
Observed Code Smells After Refactoring:
- Primitive Obsession (line 22)
```

#### Problem LCBE3.java
The class `Employee` contains constructors and features for 3 types of employees:
Ordinary, Security and Manager.
The constructors are overloaded to instantiate these 3 types of employees.
But as the number of features increases, the class expands to create a code smell.
```

Observed Code Smells:
-  Large Class (line 9-186)
```

#### Solution LCGE3.java
Applied `Extract Class` to categorize the constructors and features respectively into their own classes.
```

Refactoring Applied:
- Large Class:
    - Extract Class (line 51-138)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem LCBE4.java
Calculator class has grown in size and is now handling too much functionality to be maintainable.

```
Observed Code Smells:
- Large Class (line 1-35)
```

#### Solution LCGE4.java
Applied Extract Class to spin off some methods into their own class TrigonometricCalculator to reduce the size and responsibility of the Calculator class

```
Refactoring Applied:
- Large Class
    - Extract Class (Calculator)
```

```
Observed Code Smells After Refactoring:
- None
```