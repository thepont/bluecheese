## Code Smell: Data Clumps
### Description
Sometimes different parts of the code contain identical groups of variables (such as parameters for connecting to a database).

#### Problem DCBE1.java
The methods in `FlightBookingBad` seem to have the parameters `customerFirstName`, `customerLastName`, `PNR` repeated a lot. Also the class seems to have a lot of primitive values. The number of parameters in some cases exceed 4 parameters.
The customer verification is also duplicated multiple times.

```
Observed Code Smells:
- Data Clumps (lines 15, 28, 38, 46) 
- Primitive Obsession (lines 4-11)
- Long Parameter List (lines 15, 28, 38, 46) 
- Duplicate Code (lines 30-31, 39-40, 47-48)
```

#### Solution DCGE1.java
Applied `Extract Class` on customer related details and travel related details. Also implemented `Introduce Parameter Objects` to replace multiple parameters with a single object. 

```
Refactoring Applied:
- Data Clumps
    - Extract Class (ContactGood, CustomerGood, BookingReferenceGood, TravelPlaceGood)
    - Introduce Parameter Objects (BookingReferenceGood, ContactGood, TravelPlaceGood in method parameters.)
- Primitive Obsession
    - Introduce Parameter Objects (BookingReferenceGood, ContactGood, TravelPlaceGood in method parameters.)
- Long Parameter List
    - Introduce Parameter Objects (BookingReferenceGood, ContactGood, TravelPlaceGood in method parameters.)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem DCBE2.java
The methods in `CompareWeatherPatternsBad`, `isWeather1Greater` and `averageAcross2Weathers` seems to clump `weather1List` and `weather1List`. 
Also the weather is represented by a list of primitives.
```
Observed Code Smells:
- Data Clumps (lines 41 and 53) 
- Primitive Obsession (lines 4-5)
```

#### Solution DCGE2.java
Applied `Preserve Whole Object` to pass the entire object instead of passing individual parameters.

```
Refactoring Applied:
- Data Clumps
    - Preserve Whole Object (TwinCityWeatherSeriesGood object as parameters).
```

```
Observed Code Smells After Refactoring:
- Primitive Obsession (lines 4-5)
- Feature Envy (lines 42-43 & 56-57)
```

#### Problem DCBE3.java
The methods in `InterestCalculatorBad`, `calculateSimpleInterest` and `calculateCompoundInterest` seem to clump `principal`, `rate`, `time` together. 

```
Observed Code Smells:
- Data Clumps (lines 71 and 75) 
```

#### Solution
`Ignore` the data clumps in this case as passing in the parameters can be considered better than passing in the `ILoanValuesBad`. Passing the object increases
the coupling between the classes creating an undesirable dependency.

```
Refactoring Applied:
- None
```

```
Observed Code Smells After Refactoring:
- None (Ignore Data Clumps)
```

#### Problem DCBE4.java
There are several pair of start and end dates of type `Date` appearing in the parameters. 
No matter it is a start date for amounts, checking for overlaps or leases, they are a pair of data which can be introduced 
as a `Paramter Object`.

```
Observed Code Smells:
-  Data Clumps (line 12-13, line 16, line 26, line 36, line 46, line 55), where parameters being passed into functions,
the pair of start and end of type Date appearing as redundancy.
-  Long Parameter list (line 14, 57).
```

#### Solution DCGE4.java
Apply `Introduce Parameter Object` by creating a new class called `DateRange`, 
encapsulating both start and end of type Date and check for overlaps by passing in another DataRange instance.

```
Refactoring Applied:
- Data Clumps:
    - Introduce Parameter Object (line 7-21, line 28, line 32, line 42, line 52, line 67)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem DCBE5.java
The connection variables for the MongoDB connection are an identical group of variables that need to be grouped together.

```
Observed Code Smells:
- Data Clumps (lines 2-5) 
```

#### Solution DCGE5.java
Applied `Extract Class` on the variables to create their own class to be passed to the actual connecting class

```
Refactoring Applied:
- Data Clumps
    - Extract Class (MongoDbConnectionVariables)
```

```
Observed Code Smells After Refactoring:
- None
```