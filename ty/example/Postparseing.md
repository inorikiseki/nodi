### How I came to the concept to postparsing and how it's going to be more close to AI. 

So I will reached that it's possible to translate add(x,y) into add x y.
and even we construtct this sentence.
"Hello, world!"@ capitalize, insert '#' at [1 2 3], echo --dynamic.

if we can break a long identifier into servel words, so if it's possible to break words into letter level. 
And make the compiler look into the words synthesis to understand the words meaning?

that's to say, instead of using '--dynamic', we can use 'dynamically'. 

if we can make it words level, why cant we make it it letters or characters level as we human do.

### add(x,y) --> add x y --> addxy

1. fn add(x,y){x + y}
2. add x y =>
    X + y
3. addxy => x + y


this is showing that we want to make it look into the words. (NOTICE: this may introduce divergence. )

first to achieve this, the complier should not be just seeing a IDENTIFIER, it will try to comprise up the identifer itself as
it needs to. 

if we partition letters into tokens, which we fix the rules so that the complier actually can 'see'.

but it cant see more for pairs(0 1) than pair(dynamic dynamically). 
actually that's us who made it impossible for it to looking into it. cause we stop it at token level or words level.
cause we trimed off these info into a simple IDENTIFIER 
concept. As compiler see it in no difference with seeing (A B)

if it see in a letter level which as humans do, it's becaming interesting. it becomes looking like humanish.
we can write it in more humanly. 

Why we can't make it letters level is simply for complexity? We made this restrictions in our handmade langs, but not these Ai model trained. so the latter becomes more intelligent in a way. But with deeper understanding of how the latter is constructed, we coudld further promotes the controlity or reach a same or even higher goal from a little different approach.

so how compiler would know even without parsing it first?
Well, if no parsing the compiler would not even to tell the difference or understand the meaning between a 
whole sentence and a single letter. (atually it's not precise cause difference lies objectively. but it's true, if the 
subject wont be able to tell.) 

It needs to be trained or just-in-time trained.
no matter what the info it takes in, it needs to do the absorsion. 

### Anyway, the cotex is stopping the traditional tokenization. We need come up of a more powerful and flexible way for doing this.

See it in a raw way, and fetch it when there is the need to do so. 
(Which means we should see the charaters equally in a way before its meaning is formed out.
There is no need for apply a traditional tokenization stategy anymore, which is not mean we 
eliminate it all out, but we need a more flexible and powerful alternative strategies.
Furthermore, maybe instead of parsing to a tree, we may parsing it to a more complex 
multi-dimensional sementic gragh. Here we may not partitioning the tokenizing and parsing anymore.)

here is an anology.

but does that introduce an unsolvable complexity.

1. There are infinit numbers of numbers. Never a computer can list all of them out.
But computer could manage in a way. Cause we dont need to list them all out, we list part of it 
as we need.
2. Ray tracing. Instead of go from the light to trace the light from the light to the eyes.
we trace backwards, we tracing ray from eyes to light.

### So what's the training? A system needs ignition to start up.
You need to fire the engine. and you need give a spark to let the gas to lit it up even it's more gloring.
The training stage is an ignition. 




### Is there a way of just outputing "Hello, world!" even without ""

I wish to see a day I write it in this form.
.../main.xxx
Hello, world!

that's all, causing our compiler is even smart enough to do that we want to do.
(It's not always valid for ambigiousity.)


echo "Hello, world!".

set position

set position at _screen_pos: vec2


display a button at (200, 300)
  button@componet.ui::new@ set position at (200, 300).
  button@xxx/ui::(pos = 200,300). // An effective written version
  (button@xxx/ui)@ set position at (200, 300).
  '(button@xxx/ui)(pos = 200,300).
  xxx::ui::button::new(pos = 200,300).
  (new@meta button@ui/xxx)::(pos= 200,300).

###loop sentence.
for each x in [1 2 3 4 5]
  echo x.

print propertity@ui/button, ln, next(@button@ui@properties)...

print it util next is None. 

list = '[1 2 3 4 5 ..] 
sum = 0 

Get one from list(:[int], c'this is a comment') randomly, add it to sum, next@list...

there should be a more precise and minimal core featured version while with a liberal version. 
