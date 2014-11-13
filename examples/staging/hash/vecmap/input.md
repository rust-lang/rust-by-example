`VecMap` can be very useful in certain cases. 
You can use it anywhere you would use a `HashMap<uint, _>`. 
Internally, it's just a wrapper for `Vec` 
that handles growth and inconsistent indexes.

Because it uses a `Vec` for storage, 
`VecMap` will need to grow to the size of its largest key. 
So if your largest key is 50, 
a `VecMap` will take up as much space as 50 elements. 
Thus the emphasis on *small*.

Here's a good example: 
a program for managing a small apartment complex. 
Some apartments might be occupied, some might be empty. 
Maybe you'll add more later, 
and you want the program to be able to adapt without any extra work. 

{smallintmap.play}

You might not normally use `VecMap`, 
but maybe, one day, you'll have a need for it. 
And when you do need it, it will be there. And you'll know how to use it.
