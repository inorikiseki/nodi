FallingSand

World = Cell[.cap=Capacity .pos->vec2]
  Capacity => width=100 * height=100

Sand
  Type
    Water 'b= 1.
    Sand 'b= 2.
    Rock 'static
    Fire 'b= 0.5
  Static'
  Density'b -> f32

''' simplified
Static' ---> Static'static
'''

Randomize =>
  ..
    for it in World:
      it ? Empty: true  // ---> here is value, value is atomic
        World[it.pos] = Sand::Type[rand() % .count]

'''
actually you dont need to use rand() % .count
the compiler will do this for you. 
Just using Sand::Type[rand()]
multiple case: 
Sand::Type[(& 1 2)] --> Sand::Type::(& Water Sand)
With range syntax:
Sand::Type[&1..2]
What is & ? 
& means all the items are need be satified in calculation then the group is satisfied. 
| means one of the items is satisfied so it's satisfied.
sometimes it's omittable. 
'''

Simulate
  =>
    for it in World(.no('a)):
      Move it // Symbol Group, Move it=Sand::Soil(pos=1,1 density=1.2)
  below = & World[it.pos(.y-1)] World[it.pos(.x-1 .y-1)] World[it.pos(.x+1 .y-1)] //dumb way
  above = World[it.pos(.x + &(-1 0 1), .y+1)] // smart way
  Move it: Cell => 
    it ? below : 'b 
      '> swap(it, below) 
    it ? above : Density // ---> here is type, type is not atomic 
      '< swap(..) // auto-dispatch --> swap(it, above) 

::FallingSand

FallingSand::World
  .(width,height) = 480,320
  .Randomize
  .Simulate

FallingSand::World(
    width,height = 480,360
)  
  .Randomize
  .Simulate

'it => it.pos::echo

// why can't i call it this way, actullay you could do this with bidirectional context.
Simulate::FallingSand(
    World(width,height = 480,320).Randomize
)

What a program can run?
A program is a bunch of description that leads to a destined result as expected at a given state. 
To run a program is to translate these discription from a ready state into a final state. 
To compile a program is to tranlate as possible as you can but leave the unspecified state to runtime. 

C: 
enum Sand{Sand, Water, Rock, Fire}
int world[width * height]
randomize(&world)
simulate(&world)

void randomize(int** world) {
    for i in world.count:
        if world[i] = Sand.Empty
            world[i] = Sand[rand()]
}

Symbol Group has value( atomic)
Pattern all types( nonatomic )

Meta::
0
1
bit -> Meta[0 1]
byte -> bit ^ 8
i8 -> byte
bool_weak -> byte[0 1]
i16 -> byte ^ 2
i32 -> i16 ^ 4
i64 -> i32 ^ 2 
f8 -> byte
f16 -> f8 ^ 2
f32 -> f16 ^ 2
f64 -> f32 ^ 2
true
false
bool_strong -> true | false // another way of defining bool. this way it can't trans to 0,1 as long as false != 0 true !=1

Binary -> n ^ ..
n = 0 | 1
Decimal -> n ^ ..
n = | 0 1 2 3 4 5 6 7 8 9

## post statement

Sand'Particle
  Type
    Sand
    Water
    Fire
    Rock

enum Sand
sandTypeList = List <aquireComplierSupport>Sand::Type
sandTypeStringList = List <aquireComplierSupport>Sand::Type
Sand e
e::Type = Sand::Type::Water
e.simulate(World=[e])
e simulate(World = [e])

A type is just the things that the value is hanging up. It's need to be determined later.
So if you are clear of this, there is no need for declaring type.

bool -> [0 1]

to avoid write verbose code, you need 'lisp'. to process things as a list or group.
the major list has two type. 'any' 'all' as E or A(downside) in predication.

let's say we have this code. Can it run? The answer is yes. if the compiler is smart enough.

it -> bool
it ? any[0 1] : '=
    echo 'yes

may written as due to scoping:
it -> bool
  . ? any[0 1] : '=
    echo 'yes

>>> YES
Why?
bool -> any[0 1] // or bool -> [| 0 1]
Since bool is any of [0,1]
it must be one of 0, 1
so it must equal to 0 or 1
so it must be yes.

is it pratical?

Typing is to manipulate multiple possible states.
Evaluation is to conclude the deterministic state into a expected simplified value.

Why 0 1 is value?
Why bool is type?
That's the question.
You can see the nature.

Identifier, Specifier, Identity, Path,

Identifier ::
Sand Empty
Specifier ::
'sand 'empty
Identity ::
Paricle'sand Paricle'empty FallingSand::Particle'water
Alias ::
Sand'Particle
Path ::
FallingSand::Particle::Type

