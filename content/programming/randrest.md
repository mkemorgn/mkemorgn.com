
+++
title = "Random Restaurant Generator"
template = "random_restaurant.html"
+++

The issue is: everytime I ask my wife what she wants to eat, she says some variation
of "I don't know, what do you want?".  
<br>
My solution: have the computer make the decision for us.  
<br>
This program runs from the command line.  When you run it, you have to enter
the location you want it to search in (i'd recommend using a zip code for best
results.) After that, you can enter things to filter the search results.  You 
can enter things like "tacos", "italian", etc.  This is an optional field, so 
you can just press enter to skip it.  
<br>
When the program runs, it grabs all the restaurants that fit the search criteria
and puts them into an array.  After that, I use the random library to select one
at "random".  As a user, the only thing you'll get back is one restaurant.  
<br>
I'd like to build this out further just for fun.  I wrote it all with python so 
it's super easy to understand what's going on and it's easy to tinker with.  
<br>
[randrest repo](https://github.com/mkemorgn/randrest)
