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

...(continue)

