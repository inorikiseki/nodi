based on type of cpp

here is some supplement.

type should include its context.

so a more complete form of referring a type maybe.

meta
    xxx
        bool: [1 2]

bool@xxx.meta: [1 2]

And is it enough for a coding language to just considering on determined and undetermined?

would we distinguish rvalue, lvalue? const or mutable varible ?
Are those the things this lang's level at?

or is the rule = for 

1. dynamic typed or static typed is not that important cause we could specify the rule for 
each workspace. causing what's change is just the variable pointing, not the value itself.
We can just write code in the comfortable way in that area.

if it's static typed.
@'static typed'
a = 1
then a = 2 ok, a = 'a' not ok. 
@'dynamic typed'
a = 1
a = 2 ok
a = 'a' ok

let a = 1; ==> a : 1
let mut a = 1; a = 1
can we shadow? and why we shadow? 

@'static typed'
let a = 1; ok
let a = 2; ok
let a = 'a'; ok

a = 1
a = 2 ok
a = 'a' not ok
but
a' = 'a' ok
we have specifier, the anymous.. specifier is for stating that it's just a new variable, which is just like shadow.

a = 1
    a = 2
    a' = 3
echo a
> 2

no, we want it to be this actually.

a = 1
    a = 2
    a' = 3
echo a
> 1

if we want to change a 

a = 1
    a@.. = 2
echo a
> 2

if 
a = 1
a : 2 // is this valid? can we make = into : maybe no

somewhere
    somemodule
        a = 1
            a = 3
        a' : 5
            a = 6 // well what a = 6 to do with a' : 5, seems like a file has sub file.
    a = 3
        a' : 4
        echo a @../somemodule/

echo a @somewhere
echo a @somemodule.somewhere
echo a @somewhere/somemodule/'a = 1'/
echo a @'a : 5'.somemodule.somewhere

###  well what a = 6 to do with a' : 5, seems like a file has sub file. it's weird

well, that's the modifier and defining sub zone at least, and why can't it be?

echo it
    it@.. = "hello world"

but that's little verbose for typing '@..'

So, actually it should let the leader node to be the 0's sentence. 
example:

hello, world! // ! will be execute, before executing it'll check the sub zone. 
    echo hello
    echo world
        hello@../'echo hello' = "hello"
        world = "world"
    ehco marker
        marker = '!'

there could have a special node. which have control one all the other node path.

:: belonging holders, which make the subnode path flat. subnode wont have their own path rights.

hello, world! ::
    echo hello
    echo world
    echo marker
        hello = "hello"
        world = "world"
        marker = '!'

while you should not write the above code.

even this is better. 

Hello, world
    @: echo "Hello, world!"

Greet the world
    @: 'Hello, world'@..

Greet the world!

> Hello, world!

Greet _someone
    echo "Hi, <someone>! "

Tom = "Bob" 

Greet Tom! // you need to use <Symbol> to get the comptime info. 

> Hi, Tom! 

Greet_someone by saying_what
    echo "{what}, {someone}! " 

Tom = "Bob" 

Greet Tom by saying "Hello".

> Hello, Bob!

_ create a slot, or you think it create a object(subject) verb_ then the object

print_it
    echo "{it}"

print "Hello world".

specify a type constraintor.

print_it: str

return type? it's not a function. it's a sentence.
=> makes a function

result of add_x to_y but cannot exceeds_limit
    x, y, limit, result : number
    => // some nodes will pass the belongs so no need to use result@..
        result : // as the sentence return value after =>
            max of limit and x + y

a = result of add  1 to 5 but cannot exceeds 10


actually you should write..

add_x and_y with upper limit 
    x, y, limit :  number
    => : number
        max(limit, x + y)

result = add 1 and 10 with upper limit(10)

more concise and general way as you would like.

math@std ::
    clamp(it, min, max),
    clamp_it in_(min, max),
    clamp_it from_min to_max,
    clamp_it in range(min, max)s
        it, min, max : number
        => : number
            max@math(min, min@math(it, max))

points = get integer from input.
result = clamp(points, 0, 100).
result = clamp points in (0, 100).
result = clamp points from 0 to 100.
result = clamp points in range(0, 100).

what if i really need a _ ? use '_

### path detailed

path = 'root/some/level3'

### parenthesis

priority
() [] {} <> | |  !() ![] !{} !<> !| |

### 

you dont not write this code, except for you are in natrual feature.

x += n : 'math,
increase x by n : 'natrual
  x'mut, n : number
  =>
    x = x + n

you can always just use x += n, instead of 'increase x by n', 
the compiler could translate between different features of 'math of 'natrual 
into the way the viewr would like. 
As you can see, ': is not just for marking type, it's actually for designate the 
unchanging properties that the node would adhere to,
but '= would let it changable. 
it can have a value or a type, but can have multiple tags

x : number, 'std
x : 'height
x = 12
x := float type shrink.
x := any type expand.
  ` `