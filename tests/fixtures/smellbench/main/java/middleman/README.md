## Code Smell: Middle Man
### Description
If a class performs only one action, delegating work to another class, why does it exist at all?

#### Problem MMBE1.java
`FactoryBad` acts a middle man, delegating all work to `FactorySupervisorBad`. It itself does not seem to have much significance.

```
Observed Code Smells:
- Middle Man (lines 42-59)
```

#### Solution MMBE1.java
Used `Remove Middle Man`. Directly accessed delegate methods of `FactorySupervisorGood` from `FactoryWorkerGood`. This eliminates the need for `FactoryBad/FactoryGood` leading it to be dead code which we can remove.

```
Refactoring Applied:
- Middle Man
    - Remove Middle Man (removed FactoryBad/FactoryGood)
```

```
Observed Code Smells After Refactoring:
- None
```

#### Problem MMIE1.java
The class LucyChart is a proxy to gate keeping all the original servicing objects: Draw, Move and Resize.
The class LucyChart is doing little other than delegating all the works to other classes.
The class LucyChart is a 'middle man' in this case.

````
Observed Code Smells:
-  Middle Man (line 60-91)
  - In this case, the coding follows the proxy pattern, where the 'middle man' code smell can be ignored.
````

#### Problem MMBE2.java
`ProductHandler` acts as a middleman to `ProductManager` by giving it access to only what it needs.

```
Observed Code Smells:
- Middle Man (lines 55-66)
- Data Class (line 55-66)
```

#### Solution MMGE2.java
Used `Remove Middle Man` by removing `ProductHandler` and letting `ProductManager` directly use `Product` methods

```
Refactoring Applied:
- Middle Man
    - Remove Middle Man (removed ProductHandler)
```

```
Observed Code Smells After Refactoring:
- None
```