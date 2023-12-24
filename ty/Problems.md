## The major problems that needs a overkill

1. Path, context, category, identity.

2. Version control, change tracing. space-partion.

3. postpropagation, ray tracing.  finite verification. 


version control. 

for file F, the size of it is S.
how to track the size of change in it. it's like rendering terrain in game. do partition.
the atom is unit 1.

        S
  s/2     s/2
s/4 s/4  s/4 s/4

index it.

        0
   1        2
3    4    5   6

or binary partition. we get a number standing the chunk of the changed area. then leave the update content.
        0
    00     01
100  101  110  111

here we came up that:
the partition level = the count of the bit. [0 1] ^ n . the bigger n is, the level of particing is. the changed area is more small.
for a 2^m bits file. we only need at most m bits to trace it into a single byte. 
the index is in range[0] - [2^m].

when the bit cames to m(or about) we can know that we there is only a single letter changed.
if is came to (m - 3) to know a segment of segment of about (2^3) bits area is changed. (which is a byte)

how many changes there would be? 


<change area index> : <changed content>

now the problem came back to category and path. how do we manage indexing? 

==== changes record in low level changes.

change may record in change record.

for a change N, whose level is 2. the third block. which(the indexing rule) is related to the utf-n coding? 

for a change we record it as: for this is N<2, 3>: content

<level of change, the block ordered-id>: change content

here a another change M, which level is 3, and the count is 5, which the the left child of N. 
so we need to record that change in N. (the number is more relational in binary)

N<2,3>(ob11): content 
    M<3,5>(0b101): content 


### space partition
for a space S, we can locate a area of it with <LEVEL, ORDER>which can be encoded into a LEVEL bits of binary.

for a catefory N
any category that has the same lower bit of N is the child area of N. which is important in describing belonging relationship.
useful and effective for LANGUAGE simbol interpretation. 


### semantic space partition

THIS kind of method is aglluentative in the concept of language. (nian zhuo) so agulluentative is good for describing realtional concept. this is why a set of words comprise a sub concept of a less set of words.
example: "red apple" is subset of "apple"

for concept N and M, 
if N is 11, M is 0111, we may see M is a subset of N
if N is 11, M is 0100, we can see M is not a subset of M.
while M N can come up of anther space.
For a character 2 ^ 8 bits. or  one of 26 letters. we partition the semantic space by these letters.

if we consider N is the type, the the subarea is the value area of that type.

a infinite type 11, the value or subtype of it could be 110 111

if a type is finite, we can make bits constraint . so there only finite values of it.

when define a type of N bits, if the determinisc of  it is N-1, the particing rule is 2. 
so the value of it could be 2 cases.

so the partition more complete way is comprised of these factors.
<S: Space> = <P: partition><L: level><I: level id>

for english words(case insensitive) its partition is 26. The level is numbers of letters. the id is the word itself. 

we lay on binary partition.

more conceptually the partition itself varies in context as well.

===
may think as this
anologous to radix and numbers. 

<S: Space> = <R: Radix><P: Precision><D: Distribution>

the number is a partitioning of number range(space). 

we can compare it to floats. 

The partitioning is infinite, but the subject actually could be finite. ????
This links to the the finite against infinite (The ray tracing priciple things)
and more... which leads to phylosophy, we may exist based on finite computing power, cause
we see a finite area of it.


that means there is a constant upper limit in a way. 
that's also similar to hash.

we can thus create a relational semantic space and do semantic analysis in a constant limit. 
as long as the relational concepts are mapped to the the layout in the memory or path space.
Thus we dont need to do a unconstant look up.




