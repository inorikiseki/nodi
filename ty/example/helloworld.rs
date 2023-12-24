'''
hello world code
'''

echo "Hello world!"
>>> Hello world
'''
Typing
'''

another_type = bool int
the_third_type = another_type(true)
the_third_type o
o = 1
echo o

>>> true 1

'''Control Flow'''

n: bool = 1
n ? 0 
  '= echo "equal"
  '!= echo "not equal"

n: [i8] = [1 2 3 4 5]
n ? [2 4 5] 
  has any => echo "{n} contains one of {.1} 
  has all => echo "{.0} contains all of {.1}"

a = n has_one_of [2 4]

who: Any has_one_of what: Any => 
  any [
    it == other
    for it in who
    for other in what
  ]

b = n has_all_of [2 3]

b = n in [1..10]

'''loop'''

loop it =>
it::
  for i in [..]
    i ? 10 '= 
      break

loop{echo 'loop} 

loop'outer {
  loop'inner {
    break'outer
  }
}

val = {a: int b: int}
val a // a is value. so it's deterministic . it will be determined either with defualt initialization or just take whatever in the memory.
echo a
// = for varible : for const 
Red: Color{r: i32 g: 0 b: 0}
Pen = {size: int color: Color}
SmallPen = Pen{size: 1 color: color}
RedPen = Pen{size: 1 color: Red}
Material: any[Wood Plastic]
MaterialPen: Material Pen
MaterialPen mp
mp::Material = Wood
mp.size = 1
mp.color = Red{r: 100}
WoodSlimRedPen: mp
echo WoodSlimRedPen