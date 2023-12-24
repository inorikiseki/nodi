## here is about how to design a language named ty

1. structure abstraction

- basic structure is just 'Tree', all it starts with a tree
- Hierachy Design

```yml
example
Root
    node
        node
    node
        node
        node
    node
Another
    node
        ...
    node
    ...
    node
```

2. more specific example

```yml
shape
    area
    rectangle 
        width
        height
        colored
    square
        edge
    circle
        radius
```

its a tree constructed on different relationships. we need to specify the relation of them when needed.

here is a example:

```yml
shape
    + area
    <-rectangle
        - width
        - height
        : colored
    square->rectangle
        - edge
    ->circle
        - radius
```
now maybe we can somehow get a better understanding of the tree.
- furthermore.
```yml
shape 
    + area
        > s:rectangle
            =>s.width * s.height
        & s:square
            =>s.edge * s.edge
        & s:circle
            =>PI*s.radius*s.radius
    <-rectangle
        - width
        & height
        : colored
    square -> rectangle
        - edge
    ->circle
        - radius
    triangle --> rectangle
        - base | width
        & height
        area => .base * .height * 0.5
        . area s:rectangle => s.area*0.5
```
- let us try to  write it in a compact style

```
shape{+area{>s{:rectangle=>s.width*s.height:square=>s.edge*s.edge:circle=>PI*s.radius.s.radius}}<-rectangle{-width&height:colored}square->rectangle{-edge}<-circle-->square{-radius|edge}triangle-->rectangle{-base|width&height+area>s=>s.area*0.5}}
main{s:shape.rectangle={1,2} print s.area}

```

shape{area{s{rectangle=>s.width*s.height square=>s.edge*s.edge circle=>PI*s.radius * s.radius}} rectangle{width height colored}square rectangle{edge}circle square{radius edge}triangle rectangle{base width height area s=>s.area*0.5}}
main{s:shape.rectangle={1,2} print s.area}

- may not easy for reading. a litte modification.
```
shape{+area{'>s{'':rectangle=>s.width*s.height:square=>s.edge*s.edge:circle=>PI*s.radius.s.radius}''}'<-rectangle{'-width&height:colored}'square->rectangle{'-edge}'<-circle-->square{'-radius|edge}'triangle-->rectangle{'-base|width&height+area>s=>s.area*0.5}'}
main{s:shape.rectangle={'1,2}' print s.area}

maybe more readable when renderred with colors.
lamda in ty: <x=>x*x
add< a b => a+b
sub< a b => a-b
mul,div::operator
a mul b = a * b
a div b = a / b
'*','/'::deleted
```
- we can break this tree into small trees as we want.
```yml
# 1
shape <-
    rectangle
        <- square
        <-- triangle
    triangle # -->rectangle
    # square ->rectangle
    circle --> square # there shows two way using -->|<--
shape +area 
shape .area 
    < s
        :rectangle =>s.width*s.height
        :square =>s.edge*s.edge
        :circle =>s.area*pi
        :triangle =>s.area*0.5
shape.
    rectangle -width -height
    triangle -base -height
    square -edge
    circle -radius

# define a new shape

shape2d @shape -->shape # @ override a original node
shape3d -->shape

# now
# shape
#   <-- shape2d
#       <- rectangle &square &circle &triagle
#   <-- shape3d

shape3d<- cube &plain &sphere

```
relation reference:
```yml
?shape<-rectangle # true
?rectangle<-square # true
?circle-->square #true
?circle-->rectangle #flase --> dont propogate
?square->shape #true -> propagates
```

- define relations, to let things more flexible . maybe we should leave a chance for user to custom relations.

- how to use the above tree as some real code.
```yml
main:
    shape<< shape.rectangle<< 4,5
    print test_shape.area
    new_shape: shape.as triangle
    print new_shape
    shape<< shape.circle << 4
    print shape.area
    shape_new<< shape.circle << 1
    print shape_new.area
```
can we use shape<<1 ? well shape dont have - relation
but shape has - via <- and <-- you can put data into shape
```yml
shape<< 1 # shape has one - relation on, shape can behave like square or circle 
shape.area # but which to use ? by default the -> relation <->
c:shape.as circle
c.area
shape<< 1,2
shape.area
```

back relation like - + which searches for the parent in the tree is good for forming a tree the basic architecture.
but has its limmitations 
with forward relation like -> --> and > we can break the monologue of a tree, bring it into a graph with more complex structre.

All is Realtions of Nodes

### one more application

creatrue
     id
    -hp
    -job --> rel::has
        -equipment
        :fighter=_,10
            sword
        :archer=_,8
            bow&arrow
        :wizard=_,7
            wand
    -party
        -attackable
        :enemy
            true
        :alley
            false
    +display>job=>print it.id .hp .equipment

main=>creature::fighter<<123 display



