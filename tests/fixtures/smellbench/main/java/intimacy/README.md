## Code Smell: Inappropriate Intimacy
### Description
One class uses the internal fields and methods of another class.

#### Problem IIBE1.java
`SubjectMarksBad` fields are not private. This leads its delegate `EnglishBad` to access its internal fields and modify them as the delegate wants. 
Also, although `EnglishBad` extends `SubjectMarksBad`, it calls on its `getGrade()` using a delegator. 
Also, `TeacherBad` is able to access the internal marks field and edit it even though it is not defined to be an intended behaviour by the developer.
`subjectTeacherList` is a temporary field never used in the class.
```
Observed Code Smells:
- Inappropriate Intimacy (lines 38, 43, 58)
- Temporary Field (line 31)
```

#### Solution IIGE1.java
Applied `Replace Delegation with Inheritance` to reduce duplication between `EnglishGood` and `SubjectMarksGood`. Applied `Encapsulate Field` . This is not exactly a treatment prescribed in Refactoring Guru but step 1 of `Move Field`.

```
Refactoring Applied:
- Inappropriate Intimacy
    - Replace Delegation with Inheritance (Removed delegator from subclass)
    - Move Field (made the fields private by encapsulating them)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem IIBE2.java
Imagine this coffee machine is making 4 types of coffee: 

Espresso (hot only), 
Americano(iced and hot),
Latte(iced and hot), 
CafeMocha(iced and hot).

The interface is Coffee, which is firstly implemented by an AbstractCoffee class and then these 4 types of coffees.

The problem lies on when the machine starts to make a CafeMocha, it grabs a Latte first and continue working on 
top of that Latte; Even though the final ingredients are correct, private fields and methods from class `Latte` are 
constantly used in class `CafeMocha`. Same wrong pops out for Espresso and Americano as well.
```
Observed Code Smells:
- Inappropriate Intimacy (line 78, line 82, line 91, line 131, line 135, line 145)
                         
```

#### Solution IIGE2.java
Applied `Replace Delegate with Inheritance` to get the thing done easier and get rid of the `Inappropriate Intimacy`.
```

Refactoring Applied:
- Inappropriate Intimacy
  - Replace Delegate with Inheritance (lines 78, line 88, line 100, line 109, lines 138, line 141, line 149, line 154, 
                                       line 159) 
```

```
Observed Code Smells After Refactoring:
- None.
```

#### Problem IIBE3.java
`Library` class uses the internal fields of `Book` class as they are marked as public.
```
Observed Code Smells:
- Inappropriate Intimacy (lines 38, 43, 44)
```

#### Solution IIGE3.java
Applied `Hide Delegate` to make field private and create a getter for the field instead to make the relation "official" instead of directly mutating the field. This is step 2 of the Extract Class and Hide Delegate treatment but since only a field is being accessed, the `Extract Class` part seems unecessary and making the field private from public seems to be enough.

```
Refactoring Applied:
- Inappropriate Intimacy
    - Hide Delegate
```

```
Observed Code Smells After Refactoring:
- None
```
