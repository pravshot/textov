# textov
Group Name: textov

Group members: 
Praveen Kalva spkalva3,
Anushri Mittal anushri6,
Akanksha Kumar kumar65,
Arul Viswanathan arulv2

Project intro:
Our project uses Markov chains to generate sentences from a text dataset. textov will read in user inputted text files and generate uniquely ordered text that resembles the words in the file. Uses parallelism and sparse matrices for efficient memory and runtime.

Goals:
- Create a Markov Chain map off of the words in a text file
- Be able to use that Markov Chain to generate sentences 

Why we chose it:
- We found markov chains interesting and we thought the application of it on text would be a fun project idea.

System Overview:
- Read from text file
- Cleaning the data
- Create Markov Chain mapping using sparse matrix and data
- Create stochaistic model using weighted probability from the sparse matrix to simulate/generate sentence
- Repeat for more sentences
- Add pararallelism/concurrency to speed up creating the markov chain
- Eventually, add on higher order markov chains for different results (more random to more deterministic)
- If we have time, create a web app/ui 

Possible Challenges:
- Dealing with errors when reading data and creating Markov chain
- Using pararallelism without running into errors with shared memory and ownership
- Edge cases with data formatting/cleaning

References:
- We found inspiration from this article: https://chalkdustmagazine.com/features/fun-with-markov-chains/ .
