# nodic
main node with sub nodes groups

a line can be a group 
group count can vary 
single node 
pair node
group descent evaluating.
context-sensitive main node select, global default is the first when 2

## 2 node case. pair adjascency
'''
print x   -> print(x)
x increase -> increase(x)
'''
which is the main node? how to differentiate the relations of the two cases?
using weight. editor can show the weight. atomic has the least weight.
simple cases: weight by group weight. weighted parsing and sementics


= x 0
= a 1
print x 'x is {x}'
+= x = x + 1
increase x x += 1

print and increase has higher(or lower due to calculation diff) weight, so we can know how to run it. when there comes a vague, 
it warns or errors due to needs or expectations.

### group types
- natrual group
1. paired | doubled
```
print x
x increase
negate x
x 10
10 x
20 30
fn main

```


2. triple 
```
print x y
add x y
x add y
x y add

```
3.  lined -- group such as in a line
```
print x y z
transform position rotation scale
vec2 x y
color x y z w
color r g b a
```

4. blocked 
Player
    Transform
        position
            x,y
        scale
            x,y
        rotation float
    Sprite
    Controller
- **marked** using other node to mark
1. 
begin 
    code code code
end
2. (a,b,c)
3. [a,b,c]
4. 
```
{
    a:{
        b:{

        }
    }
}
```
- **specified**
print a b c

call print'3+ -> print(a,b.c)
call print'2+ -> print(a,b)
call print'2,3 -> print(b,c)

may come up more range specifying.

x y z increase

call increase'-3
...

and there are more complex specifyings.

### Sample

A 10 +
B 20 -
C 30 *
D 40 /

4 triple groups. 
there are four people A B C D
with a tag writes 10 20 30 40 
has four principle add sub mul div

now here is the code

A B C D

how to interpret it.
1. as default.
in the default global context. --> The first is main node. the main node acts as the caller.
(a) A' B C D -> using A' to mark out the main node in a group.
(b) A' is main node in A B C D
but what is A?
A 10 + :: A is main node which acts as a caller for now. the subsequence is to find out the real caller in the ' A 10 + ' group.
'+' wins out. because binary has more caller weights.

so it's A(B,C,D) --> + 20 30 40 --> (+ 90)
the result is not just 90, but a state of adding 90. if there are no other consideration. it's descent to 90.
cause it's not equaling to 20 + 30 + 40
if there is another 30, the + can't directly interpret to a binary add

let x = A B C D
let y = x 10
what it y? maybe 100
print y

notice: there would be a vague if + only possesses two node. here we think it add all node that's add-able.

in this case:
1. 
A' B C D
2. 
A 10  +'
B 20' -
C 30' *
D 40' /
3. 
+ 20 30 40

2. when other node acts as main node it is similar.

### lisp take the first as main node. a simple way is to mark the main node a ' so can flex the order.

but what does it mean when it's 
10 20 * 40
it depends on how to interpret * . the normal way is like 
x = 10 20 * 40
y = 30
z = x y
print z --> z = 30 * 10 * 20 * 40
the default explannation of a main node is to apply the rule it holds with all sub nodes equally.
so what are other cases?

#### if

x if then else
then = 10
else = 30
a = false
b = x a
print b

what is b? 

```chatgpt
    I understand. Given the provided context and treating it as a quiz, let's try to interpret the code:

    The x if then else looks like an attempt to define a conditional expression. Assuming x is a placeholder for a conditional expression, and treating it as a ternary conditional expression, the result depends on the value of a.

    If a is true, then the result would be then (which is 10).
    If a is false, then the result would be else (which is 30).
    Given that a is defined as false, the value of b would be the value associated with else, which is 30.

    So, in the context of the quiz, the value of b would be 30.
```
### We may want to explicitely konow the main node and control the interpretion. 

Add a b

what is Add a b, it's not complete if the context is unclear.
is it equal 'a+b'
x = new Add 10 20
y = new Add 30 40
z = x y

what is z?
there is too much vague.


### go more practical, considering lisp

Add' (a b)
(Add' a b)
(a Add' b)
a Add' b


### let's define
bool
num
str
id
if cond then else
while cond do
fn id args body
class id inherits body