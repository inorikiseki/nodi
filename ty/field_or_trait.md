The attributes of a object are not born within itself. What properties, fields or attributes it has are actually 
meaningful when there are the requirements or reliament on it. 
When we say 'color', there are visual systems to translate how color is, what the data stored in 'color' means.
Get rid of all the attributes that are attached to a object or a class or anything, the remain is just its
identifier.
No matter, how you design it. The attributes are just the things that linked to the 'identifier' in a way.
More often, we can't list all the 'attributes' of a item, cause it usually depends on how we view it in different 
aspects. There comes endless properties 'cause there are unforbidenably different view points.
So, instead of the traditional way, just list all the properties a item or a class has. There are actually the 
same nature, but different looking ways.


** Usually the properties and the item forms a sub-owner relations. But which is the owner?Which is the sub?
Traditionally, item is the owener. But, actually there are no actually owners. We should make it flexible enough
that we could make it 'owner' no matter which when we need. 

In a falling sand game.

CellType
    Sand
    Water
    Rock
    Fire

1.  We create a five 'identifer' or we could call 'metaitem',
but what's the relationship of them? that's the question.
Is CellType the owner?
No, we said we dont want to create a owner. Or we don't want to 
fix the relation only to a owner with its belongings.
Since it is called CellType, so there are Cells.
Sand, Water, Rock, Fire are actually Cell-s.
Right?
So, here lays other implicitly relations. Cell and Type.
Sand is actually a Cell Type.
so if we want to know the cell type of sand. what is it.
it's it self under the context of Cell Type.
Sand.Type == CellType::Sand.
We just state one statement, Why comes so many things. 
Should one statement just carry a fixed relation?
for cell in Cell.subs.iter()
    print(cell)
What should it print.
i wish to get Sand Water Rock Fire

A more reasonable way of write this, is to break it down.
Cell::
Type:
    Sand
    Rock
    Water
    Fire

2. let's see one more case.
if we want to know whether a sand can is active(movable) or static(stable).
How we specify them. Store a variable to  each of the item? 'isActive'
Thus, the traditional view on this is to came up of a struct or something similar.
Struct Cell {
    type: CellType,
    isActive: bool
}

And thus we must create the data for the instances of these in OOP or create a properies struct in rust or c.
Cell sand.
sand.type = CellType::Sand
sand.isActive = true

Cell rock.
rock.type = CellType::Rock
rock.isActive = false
...

In this way, here cames many problems. 


** A alternative way is to **

Cell::
Active -> bool:
    true'a
    false'b
Type:
    Sand =  'a
    Water = 'a
    Rock = 'b
    Fire = 'a

For bool, we could came up a shorthand.
in this way.

Cell::
Active'a -> bool
Type -> Enum:
    Sand =  'a
    Water = 'a
    Rock
    Fire = 'a


3. Here is the question due to the above design.
it brings the things that belongs to values more into types in a traditional view.
Type is complied, value is runtime.
 What's the boundary between type and value. 
This question is a query for what should be static or dynamic. 
What is compile time things or runtime things.

Or, should these be distignuished at a languge design level?
Is there need to fix the value, runtime and the type, compile time.
A class or struct itself is an extension of type. 

FallingSand::

World = Cell[.cap=Capacity .pos->vec2 ]
  Capacity => width=100 * height=100

Sand
  :Type:
    Water 'b= 1
    Sand 'b= 2
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
      it ? Empty: true
        World[it.pos] = Sand::Type[rand() % .count]

Simulate
  =>
    for it in World(.no('a)):
      Move it
  below = & World[it.pos(.y-1)] World[it.pos(.x-1 .y-1)] World[it.pos(.x+1 .y-1)] //dumb way
  above = World[it.pos(.x + &(-1 0 1), .y+1)] // smart way
  Move it: Cell =>
    it ? below : 'b
      '> swap(it, below) 
    it ? above : Density
      '< swap(..) // auto-dispatch --> swap(it, above)

FallingSand::World
  .(width,height) = 480,320
  .Randomize
  .Simulate