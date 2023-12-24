### ECS featured language
Today, when I was coding in godot thinking about game architeture and how to organise my code, then it occured to me
that to do ECS design is a good choice and I would be much comfortable of godot if it offers a good ECS approach for make 
the game. Unluckly, godot is not the one who is exerting the advantages of ECS.
However, when i thinking deeper. It also occured to me why there is no a langague doing ECS from a language level? If so, I
would definitely appreciate that language for making game. Is it possible for making a languge featuring the ECS paradigm?
What would it be like of such a language in reality?

### An exploration on the possibility
I am gonna make some basic presuasion on what a ECS languge would be like, most importantly on its syntax support.

#### Silbing Sharing

The inheritance strategy statically binds the super class and its derived class. 
Well, that's point why it sucks when you want to reuse part of the
functionality only lying in the child class.

Contravesely, The Entity-Component-System Architecture excels in its flexibility.
You can make small components and combine them into a functional entity as whatever
feature you like as you could add on. 

The program is deeply relying on reusalbility, because this feature that we could code 
wouderful software with just lines of letters.

**Reusability** **Capsulation** **Polymorphism**
That's the three most important goals for designing a language.

Here is about the **reusability**. Instead of just extending the property the super class.
We just declare of using other class properity. When constructing a entity, there not only 
the domiant node matters, but also the silbing nodes. 


Imagine we are design a space shooter game. 
Here is the contents that we should take into consider.

Ship
    Collider
    Body
        Base: Sprite
        Engine: Sprite
        Weapon: AnimatedSprite
    Emitter
        field - position
        attached - position
        emittee - Emittee
            transfrom
            sprite
            tracker
                :naive
                :straight

        EmitMode


Entity::
    Transform::vec4 vec4 vec4 vec4
    Sprite::offset texture 
    Collider::Shape Layer

we have a emitter entity who needs to know itself is where to put its emittee is and what he should emit and how it should emit.
Emitter
    field
    target
    emittee
    mode

Emitter 
    shares attached with
        who has Vec2 
    has field 
        who has Vec2
    shares emittee with 
        who has Vec2 
        and can Show
```
Emitter
    attached: share Vec2
    filed: Vec2
    emittee: share Vec2<Show>
```
```
Ship
    positon: vec2
    body: Body
    emitter: solve Emitter
```
```
let e = Emitter::new();

let s = Ship::new(e);
```

Bullet 
    can show and
    has Vec2

Ship 
    has position 
        which is vec2

Ship
    can animate

animate is Show

emitter
    is Emitter
    built under Ship

emitter emit

let a be int and equal 1  //let a: int = 1
let b be float
print the result of a added with b

define int added with float   


    cast float into int 

let emitter = Emitter<of Ship with Weapon>::built();

