Inspiration from(State Pattern)[https://rust-book.cs.brown.edu/ch17-03-oo-design-patterns.html#trade-offs-of-the-state-pattern]
Rust is writing it too verbose.
```
Post
  state: State
  content: String // how to make it private ?

  new => Post{state = Draft@State ..} //@ is reversive addressing
  add content: String => 
    content.push_str(content)
  require review => @State //delegation
  content => @State
  approve => @State

  State 'E
    Draft
      =>'approve
        err@std "can't approve before being reivewed", ln.
    PendingReview
      =>'require review'
        .= self  // . refers to the context owner
      =>'approve
        .= Published 
        // when doing something dangerous or needing attention using ! instead. When something is has vague using ?
    Published
      =>'content
        content@Post
```
new@Post
  add content="One way to write"
  require review
  publish.

Post::new, add content("Another way to write it. "), require review, publish.

usually . is for refering the object(value, deterministic) :: is for category, hierachy.



Privacy rule::
1. if depth is beyond 3 or 4(you could define the level), then it's automatically private, cause it's two deep, deep means it's inner things.
shadow means it's public to use.
so if Post is exposed, the shadow level is exposed too. so people can use. 

require review => @State
content => @State
approve => @State

As i would not like this, so the better way of writing it is? 

require review,
content,
approve => @State

//or in a single line.

require review, content, approve => @State

even they has parameters, you can assign it respectively. 

use a 'and' if you like.

require review(passing arguments), content(passing arguments) and approve(passing arguments).

when . is put the 'sentence' would be evaluated.

State 'E
  Draft
    =>'approve
      err@std "can't approve before being reivewed", ln.
  PendingReview
    =>'require review'
      .= self  // . refers to the context owner
    =>'approve
      .= Published 
      // when doing something dangerous or needing attention using ! instead. When something is has vague using ?
  Published
    =>'content
      content@Post

A alternative way of writting this would be?

:filter

State
  :Draft
    :approve =>
      err@std "can't approve before being reivewed", ln.
  :PendingReview =>
    :require review
      .= self 
    :approve
      .= Published 
  :Published =>
    :content
      content@Post

======
or put it at the parent side(it doesnt touch the node or symbol)

State :
  Draft : =>
    approve
      err@std "can't approve before being reivewed", ln.
  PendingReview : =>
    require review
      .= self 
    approve
      .= Published 
  Published : =>
    content
      content@Post


So a whole modified version would tend to be.

Post
  state: State
  content: String

  new => Post{state = Draft@State ..}
  add content: String => content.push_str(content)
  require review, approve, publish => @State
  
  State :
    Draft : =>
      approve
        err@std "can't approve before being reivewed", ln.
    PendingReview : =>
      require review
        .= self 
      approve
        .= Published 
    Published : =>
      content
        content@Post

but that's enough? No.
=====
auto marking.(Sentenced) :: marking scope

Post
  state: State
  content: String

  new => Post{state = Draft@State ..}
  add content: String => content.push_str(content)
  require review, approve, publish :': => @State
  
  State :
    Draft : =>
      '2
        err@std "can't approve before being reivewed", ln.
    PendingReview : =>
      '1
        .= Self 
      '2
        .= Published 
    Published : =>
      content
        content@Post
      '3 'do something'

the IDE would put the whole name out.

(runtime passing scope)
[addressing selective scope]
{attribute modify instance creation scope}
<compile time instruction scope>
|catching scope|
: marking scope :

about :: and @

examples:

std::out::err
=
err@out@std
=
err@std::out

post calling

1@print.

"hello world"@ echo.

"hello world"@ Decorated::echo.

"hello world"@ capitalize, echo@Decorated.

"hello world"@ capitalize, insert ',' at idx=6, echo.

String
  insert _: char at idx: u32 => implement it!
  insert '*' at idx = 6
  insert '#' at idx(i)@ i = [1 2 3]
  insert '&' at idx = [3 4 5]
=========
  insert _(character: char) at _(index: u32) =>  implement it!
  insert '*' at 6
  insert '#' at [1 2 3]

would the order change before when 1 is insert how to disambiguous

Example

'insert' symbol has --statically(-s) and --dynamically(-d) if the compiler coudl break down it to see the -ly you dont even need to use '--'

"Hello, world!"@ insert '#' at [1 2 3] -s.
>"H###ello, world"

"Hello, world!"@ insert '#' at [1 2 3] -d.
>"H#e#l#lo, world!"

--dynamic would tracing insertion index if 'a has 'bs in front of it, then 'a would add n(count of 'bs)

it equals

let's say we defined a -ly to replace --

"Hello, world!"@ insert '#' at [1 3 5] dynamic-ly.

apost = Post::new
// pretending that we need to give a path to add node even here we dont need cause apost carry the info.

apost@ add@Essay::Post content = "Nothing to say yet. ".

[1 2 3] compiler will extract the list
what if i really want to receive a list and impl it on my own.

quote it.
'[1 2 3]

## We could just simplify the design.

Post
  content: str

  new => Draft{}
  add content: str => content@Post.append(content)

  require review, approve, publish
  
  >Draft : =>
      '2
        err@std "can't approve before being reivewed", ln.
  >PendingReview : =>
      '1
        .= Self
      '2
        .= Published 
  >Published : =>
      content
        content@Post
      '3 'do something'

quoting
let's say when you need to put a lot of code in a sentence. we can using quoting to move the code to another place.

"hello world"@ capitalize, insert ('INSERTION_RULE.0) at ('INSTERSION_RULE.1), echo.
  'INSERTION_RULE => char idx
    'the logic'