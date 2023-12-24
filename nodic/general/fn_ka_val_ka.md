## Should it be the value or a function? 
Let's say we have this.
```yml
User
- name
- birth_of_date
- age
```
Should we create a age variable?  
Actually, we are able to calculate the age of a user 
without spending much costs.

So, I'd better not to make it a variable, instead, a functions.

User
1. name
2. birth_of_date
3. age => Period.between(birth_of_date, now).to_year()

So, why we need to make distinguishing of a variable and a function?

Some variable in a sense are the metadata, we can't do things without them,
but others are may deductable from these datas, like 'age'.
We usually store the latter for calculating less. Even it's deductable, it costs
more or less. So the latter is more of a cache. 
So, if it's just a cache, why not just leaving it to the compiler to decide 
if a variable should be cached or not? 

And the less or even no these unnessary data we create, the annoying and 
verbose getter and setter things would not bother or existing.

I would like put this thought in parallel with the file_or_directory.

we now encounter a relection on **FILE-DIRECTORY** and **VALUE-FUNCTION**
distinguishing. I presight the future is about to eliminating them, or bring them
into the black-box. 
