#### what is type?class?entity?field?fn?method?property?...
```rs
    # '|' means A is either a or b enum A{a,b}
A = a | b
B = c | d
C = A * B
    # C is (a,c),(a,d),(b,c),(b,d)
// lower case for atomic node, further on.
```
Pen = Size * Color
    Size = small | big
    Color = green | red
Drink = Size * Flavor
    Size = small | medium
    Flavor = apple | grape

both Pen and Drink is related to Size, what are the two Size ?
Should we combine them or just differentiate them?

if we combine them, how ? what should be taken into consideration while combining.
if there are an another Size?

    a::Size = small | medium | large
    a::Color = green | red
    a::Flavor = apple | grape

can we make it as this.

    Pen = Pen::Size * Color
        where Pen::Size = a::Size - medium
    Drink = Drink::Size * Color
        where Drink::Size  = a::Size - big
    
both has Size::small can we use them interchangably?
are Drink::small Pen::small related? Should them be related?

**imagine1** when someone is given a prize, which are a drink and a pen.
but the pen are drink can't be both the minimum level. What is the prize?

    Prize = Pen * Drink
        where Prize = Prize - Pen::small * Drink::small

**imagine2** when someone should take a small thing to school. what he could take?

    Take = Pen + Drink 
        where Take::Size = small

well, are this practial? 
type?value?
class?instance?
entity?componet?
parent?child?
enum?
what are these things?
a value of a type ? an instance of a class? a option of a enum? union?
    single route
--->集合 
```haskell

-- Define the possible sizes
data Size = Small | Medium | Large
  deriving (Show, Eq)

-- Define the possible colors
data Color = Red | Green
  deriving (Show, Eq)

-- Define the possible flavors
data Flavour = Apple | Grape
  deriving (Show, Eq)

-- Define the pen with size and color
data Pen = Pen { penSize :: Size, penColor :: Color }
  deriving (Show, Eq)

-- Define the drink with size and flavor
data Drink = Drink { drinkSize :: Size, drinkFlavour :: Flavour }
  deriving (Show, Eq)

-- Define the Prize type, which includes a Pen and a Drink
data Prize = Prize Pen Drink
  deriving (Show, Eq)

-- Function to check if the pen and drink have both minimum levels
isMinimumLevelPrize :: Prize -> Bool
isMinimumLevelPrize (Prize pen drink) =
  penSize pen /= Small || drinkSize drink /= Small

```
haskell is not enought, actually identifier is not important the type is important.
if the same type are more than one, we then add a identifier.( are the identifier itself a type? 
if so their are just types)
make it be this.
a **type**(tradition view) can constrait on **value** can **value** constrait on **type**?

Size = Small | Medium | Large
Color = Red | Green
Flavour = Apple | Grape
Pen = Size Color - medium
Drink = Size Flavour - large                           
Prize = Pen Color - small^2    //another situation Prize = Pen + Color - small
isPrize a = Size Color -> Result do
    a == Pen Color - small^2
Success = Yes = True
Error = No = False
Result = Error | Success + Yes | No
main = Result do
    put "what prize would you take"
    p = getInput
    isPrize p
        Yes => Success
        No => Error
getInput = Size Color do
    line = getLine stdin
    first + second parseWord line

<Program>       ::= <Expression> | <Expression> <Program>
<Expression>    ::= <Pattern> | 
<Statement>     ::= <Pattern> = <Pattern> 
<Pattern>       :: <Identifier> | <Identifier> <Pattern>

<Assignment>    ::= <Pattern> "=" <Expression> 
<Expression>    ::= <Pattern> | <BinaryOp> |
<BinaryOp>      ::= <Intersecion> | <Universal> | <Difference> | <Cartesian> | <Power>
<Assignment>    ::= <Pattern> "=" <Pattern>
<Intersection>  ::= <Identifier> "&" <Idenfier> 
<Universal>     ::= <Identifier> "|" <Idenfier> 
<Difference>    ::= <Identifier> "-" <Idenfier> 
<Cartesian>     ::= <Identifier> <Idenfier> 
<Power>         ::= <Identifier> "^" <Number>

<Exprresion>    ::= <Term> | <Expression> <BinaryOp> <Term>
<BinaryOp>      ::= "&" | "-" | "^" | "|" 
<Term>          ::= <Identifier> | <Number> | <Pattern>
<Pattern>       ::= <Identifier> | <Identifier> <Pattern>
<Identifier>    ::= ["_"|Letter]["_"|Letter|Digit]*
<Number>        ::= <Digit><Number>
<Letter>        ::= [a-zA-Z]
<Digit>         ::= [0-9]

<Match> <Block>