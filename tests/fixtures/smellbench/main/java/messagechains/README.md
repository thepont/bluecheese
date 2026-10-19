## Code Smell: Message Chains
### Description
In code you see a series of calls resembling $a->b()->c()->d()

#### Problem MCBE1.java
`OrderHistoryBad` tries to access person name and address by forming chains calling objects sequentially. Also, `PersonBad` and `AddressBad` have a functionality of only representing data than doing anything else.
Some instance variables such as `salutation`, `orderNumber` and `orderStatus` could have been defined in a different way than using primitives.

```
Observed Code Smells:
- Message Chains (lines 91-92)
- Primitive Obsession (lines 21, 42 & 44)
```

#### Solution MCGE1.java
Applied `Hide Delegate` to hide the calls to `AddressGood` and `PersonGood`. Also applied `Extract Method` and `Move Method` to handle printing of person and address in `OrderGood` instead of `OrderHistoryGood`.

```
Refactoring Applied:
- Message Chains
    - Hide Delegate (getAddress, getPerson)
    - Extract Method and Move Method (printOrderDetails)
```

```
Observed Code Smells After Refactoring:
- Primitive Obsession (lines 21, 42 & 44)
```

#### Problem MCBE2.java
Here the message chains arise when, an attendance manager wants to know if an employee is on shift at a particular
point in time. The attendance manager has to get the employee instance first, and then get the schedule of that employee 
finally arrives at the boolean if the employee is on shift: AttendanceManager -> Employee -> Schedule, where the class 
Employee functions like an agent, but it should not be. 
Attendance Manager should have this method directly inside its own class. 
In other words, Attendance Manager should not go through an employee to know if they are on shift.
```

Observed Code Smells:
- Message Chains (line 78, line 82)
```

#### Solution MCGE2.java
The two methods of `isOnShift()` should sit in the class Attendance Manager because to know if an employee is on shift
is the duty of the attendance manager.
```

Refactoring Applied:
- Message Chains:
    - Move Method (lines 30-33, lines 35-38)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem MCBE3.java
For the vehicle to get the address for the latest order to be delivered, it needs to go through a message chain of going to the `Distributor` object which returns a `Warehouse` object which returns an `Order` object from which the address can be retrieved.

```
Observed Code Smells:
- Message Chains (line 117)
```

#### Solution MCGE3.java
Applied `Hide Delegate` by adding a new public method `getLatestOrderDestination` to `Distributor` class to make the calls internal
Removed `getWarehouse()` method from `Distributor` so it remains private and reduces its public exposure.

```
Refactoring Applied:
- Message Chains
    - Hide Delegate (getWarehouse())
```

```
Observed Code Smells After Refactoring:
- None
```