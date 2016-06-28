The simplest error handling mechanism we will see is `panic`. It prints an 
error message, starts unwinding the task, and usually exits the program. 
Here, we explicitly called `panic` on our error condition: 

{panic.play}
