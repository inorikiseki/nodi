# Nodi language syntax descriptions.

Nodi is not a traditional language, the goal for nodi is to be an ultimately flexible language that is easy for using yet productive and effient. 

Nodi is not a competitor among all the so called coding lanugages, nodi itself is more like a super set of these languages.

The basic idea that nodi is different from the other languages is that it doesn't fix the rule preinterpret. It doesn't contain the normal lex-parse-interpret stages. 

## Basic Structure and Context concept

The basic structure of nodi is as follows.

```nodi
Node 
    Node
    Node
Node Node Node Node
    Node Node
    Node
        Node Node
        Node Node Node
    Node Node
    Node
```
Well, what does the code above mean? And what is these `Node`s?  
The code snippet shows that `nodi` is comprised of many nodes that forms a basic tree structure, which also introduce a very basic nodi concept yet very significant concept, **Context**.  

The **Context** in nodi is very different from the traditional concept in so called coding languages. 
The **Context** in nodi is very flexible and powerful than those. 
Context is not a fixed relation of parent-children or scope based.
Context is based on all the surroundings of that node.  
Context is comprised of many other Context.   
Due to complexity and distance from that node, the context the node in and what Context captures is a complex topic. We should not state it at the early stage and just go on and more infomation will show off.

## Meta Concept

Another yet most primitive and significant concept for nodi is the concept `meta`, which nodi won't exist without.  
What is meta? 

To state what `meta` is an difficult concept as well. Because, `meta` can't be fully explained by words in nature.

`meta` is any, all the concepts in the world(or universe ...). 

That seems ridiculous, but trust me, that's what `meta` really is.  

Why am I saying that `meta` is not descipable in nature?  Because, meta is beyond all the concept there are.
It's beyond yes-or-no, exist-or-not-exist and so on.  

bool: [0 1] 
what bool is to [0 1]? The meta is what bool is to [0 1] and beyond.  If the meta concept for [0 1] is bool, the 
meta for all the other including meta itself is meta. 

I know it's obscure but I couldn't state it better for now.  

[answers about context and meta by ai assitant](./answers/context_meta.md)

## Node or Meta

To call the primitives in nodi a node or a meta(Yes, a meta as if an object, not a adj.) is based on the significance you want to emphasize. Node is more on Structure, on meta is more on concept. Nevertheless, they are both capable of calling the primitives(The basic unit in nodi). Also, you could raise others you like.

Further interpretation will progressively added about `meta` or `node` the somehow same thing.

## The inner of Meta

It's too abstract for stating meta in such a way. Let's break it apart and look into it.

Meta has inner structure. What is it inner structure. You won't notice a meta's inner structure unless you step into it. The inner structure of a meta is part of the meaning of it. 

I would like to raise some basic instances or examples for this.  

meta
0 1
bool: [0 1]

First, from meta it borns 0 1 the most primitive concept after meta. With `0-1` pair we can define any other thingsthat's an inner structure of meta. `0-1` is also called a **diffum**(the basic unit for differetiaion). 

The `bool` is a meta, which is related to 0 and 1. But what's the relation.  

You may know the relation based on the name 'bool' but what if I've write it as 

xxxx: [0 1] 

The relation of the bool is that it can be either 0 or 1 as you know. But what is 'xxxx'? 
You may not tell the latter at first sight if i've put it out first. 

The relation is hidden. It's hidden until spotted out. But discussion on this topic is not now.

But you need to think which tells you the relation of bool and 0 and 1. Does it to do with '[]' ':' or something as 'bool' ?

Back on this topic.  Bool is a meta which has the inner structure. Actually it's not important as i say inner or not.

Well, another meta is also has something to do with 0 and 1, but has a different relation.

bit: [0 1]

the bit is also a meta that has to do with 0 and 1 but with different relation.

I've write them out the same format. And a bit is also the either 1 or 0.  

Is `bit` the same as `bool`? You should think on it. Give me your answer.

To explore the scope, i would define some basic concept in common coding languge to give you an insight on meta more.

```nodi
meta
0 1 
bool: [0 1]
bit: [0 1]
byte: [bit]^8
u8: byte
u16: [u8]^2
u32: [u16]^2
f8: byte
f16: [f8]^2 
f32: [f8]^4 
digit: [0 1 2 .. 9]
letter: 
    uppercase: [A B C D ..]
    lowercase: [a b c d ..]
symbol: [. , / * ..]

identifier: [<letter>_][<letter><digit>_]*
..
```
notice the above definition is just a certain aspect of the meta or node, not all its  meaning. 

## Form-Free

There is not a formal or verbal or lexing bound for a concept. That means the relation of metas are expressed in various ways and the meta itself is taking role as which meta is not bound to the simple or a fixed literal form.

All the surroundings are just a hint for interpreting the meaning of it, but not a must unless you define it to bea fixed rule meta. (I say rule meta, because every thing is just meta).

## Type or value?

What is a type or a value? A class or a instance? Or whatever these things?
Nodi, don't introduce these concept at a primitive level, although it's not excluded as it's a super set.

There only two state of a meta, which is stated by a determintor also called as 'detum'.

```nodi
detum: determined | undetermined
//also write as detum: [determined undetermined]
// like bool and bit. 

```
And there is a linkage or relation between diffum and detum. 
because the most primitive one is diffum. 
You could think everything else is derived from diffum especially more related with these binary set.
like `detum` and determined is derived from 1 and undetermined is derived from 0.
also, this apply to bool: true | false.

a meta is either determined or undetermined. 
0, 1 is a determined meta. because there wont generate divergence from 0 or 1.
bool is a undetermined meta. bool is maybe 0 or 1.

For a detum in undetermined state, it's similar to a type. For a detum in determined state it's like a value.

other undetermined metas are like `byte` `u8`. 
a determined meta is "hello"

Notice, when we say about detum or meta, we are always talking on its single aspect not all the case from deferent context. 

 for a meta hello, world!
if we treat it as a string literal, it's a detum(determined) in most context(context with string literal defined). 
but we treat it as a centence, it may be not the case. We dont know what exactly it means unless defined.

a centence is the concept we'll cover later.

(continue...)
