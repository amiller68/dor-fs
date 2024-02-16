Advancements in AI, particularly in GPT-based technologies, are having a profound effect on our understanding of tools. Regardless of where you fall on the spectrum—whether a gleeful optimist, a basilisk doomsayer, or somewhere in between—it is difficult to use GPT-based products without feeling as if something has changed in our relationship with technology. GPT has disrupted our conventional perception of technology as solely a tool—as entities utilized, rather than agents that are engaged.



Take hammers, nails, data lakes, calculators, and email servers – these are *tools* with specific, understandable use cases, workflows, and outcomes. Despite potential flaws or failures – a bent nail, a crashed email server – our interaction with these tools is straightforward and limited in scope: we use a hammer to hang a piece of art, or an email server to send a picture of it to someone. These tools have a clear purpose from the moment we pick them up until we set them down. This is a simplistic description of a complex and historical concept like 'the tool', but it captures the essence of what I'm conveying. However, this traditional understanding of tools as straightforward and purpose-driven is challenged by the advent of GPT technology, introducing a new dimension in how we interact with and perceive our tools.



Traditional tools, like hammers and email servers, represent a straightforward interaction, contrasting with the complexity introduced by GPT. GPT epitomizes a distinct and groundbreaking disruption to the old, aforementioned relationship between agents (humans) and tools (hammers, nails, email) in a way that prior AI products are not. Even the most advanced of our machine learning models and AI products prior to GPT's release are still fundamentally context-constrained—trained for and run on straightforward, albeit indescribable tasks (play chess, sort all the dogs and wolves in this corpus). They are developed, used for, and employed for the purpose of carrying out a specific task. While their functionality may not be instantly comprehensible or perceptible as that of a hammer or an email server, in practice, they fulfill a well-defined and scoped set of use cases. Just as a hammer is a tool for hanging art, these products can be seen as tools for reducing and solving complexity.



GPT is not so simple. In its most basic product implementation -- standard GPT -- it doesn't sit neatly in our description of a tool. Let's suppose I ask an instance of ChatGpt to



> "
> 
> remove comments from this code and give it back to me
> 
> // dis is me fib func i hope it works
> fn fib(1) {
> 	// wooop de wooop it swtich time
> 	switch n {
> 		// this is a BASED case
> 		1: {
> 			0
> 		},
> 		// this is where things get SPICY
> 		2: {
> 			1
> 		},
> 		// idk if there's a lang that would let me do this
> 		x: { 
> 			fib(x - 1) + fib (x - 2)
> 		}
> 	}
> }
> 
> "



This is a simple, but not entirely constrained task -- i am asking my instance to:
- differentiate between prompt (what we want it to do) from the task material (our code)
-  understand the relation between source code and comments
- parse which portions of the source code may be comments
- filter and write out the cleaned up code
For me, it responds:



> "
> 
> Certainly! Here's your code with all comments removed:
> 
> ```rust
> fn fib(1) {
> 	switch n {
> 		1: {
> 			0
> 		},
> 		2: {
> 			1
> 		},
> 		x: { 
> 			fib(x - 1) + fib (x - 2)
> 		}
> 	}
> }
> ```
> Please note that this code appears to contain a few syntax errors and may not function as intended. If you need help debugging or correcting it, feel free to ask!
>
> "



Though we know GPT is just operating on my input one token at a time, at a higher level of abstraction it attempts to interpret my intent, carry out its execution, and return a result. It even goes as far to re-interpret and extend my will by (wrongly) identifying my code as Rust, suggesting that it may contain errors, and prompting me to ask for help with syntax. Now consider if I follow up by asking:



> "
> Fix up this code. Also what projects might this example serve as a good departure from for someone new to coding? What fun things can I do with infinite sequences?
> "



It responds:



> "
> To fix up the code, it seems like you want a function to calculate the Fibonacci sequence in Rust. The Fibonacci sequence is a series of numbers where each number is the sum of the two preceding ones, usually starting with 0 and 1. Here's a corrected version of the code in Rust:
> 
> ```rust
> fn fib(n: u32) -> u32 {
> 	match n {
>         0 => 0,
>         1 => 1,
> 	    _ => fib(n - 1) + fib(n - 2),     
> 	}
> }  
> 
> fn main() {
> 	let result = fib(10); 
> 	// Example: Calculate the 10th Fibonacci number
> 	println!("The 10th Fibonacci number is {}", result); 
> }
> ```
> 
> This code defines a recursive function `fib` that calculates the `n`th number in the Fibonacci sequence.
> 
> As for projects that this example might serve as a good departure point for someone new to coding, here are a few suggestions:
> 
> 1. **Sequence Generators**: Extend this concept to generate other mathematical sequences, like prime numbers, factorials, or Pell numbers.
>  ...
> "



This interaction with ChatGPT illustrates how interacting with GPT diverges from interaction with other 'tools' -- GPT forms a collaborative relationship with its users. Not only do I direct the instance towards a high level intent as expressed in my instructions, the agent constrains its response and my possible courses of action in meaningful ways:
- it extracts an intent to use rust from my earliest snippet and corrects it, both doing what I asked and embarking on something of a "self" directed path
- it sets me down a path of embarking on one of several projects that it deems to be in my stated interest, but constrained by its own knowledge about the world and myself
It's not clear that I am _using_ my instance in the way that I might use a hammer -- whereas with the hammer I am aiming toward a seemingly definite and self-actualized result, when _interacting_ with my GPT instance this character of myself is loosened. I may choose to or not to act on the advice and suggestions of my instance, but I cannot own or identify with its ideas and suggestions -- they didn't come from me of course. If I were to see this interaction through it would resemble embarking on a relationship with this instance, one in which I and it are directing our intent on each other, extending and defining our capabilities within the context of a mutual formation.



As a phenomenon, GPT, as opposed to traditional tools, is distinct in that it doesn't merely extend my will and agency but transforms, extrudes, and extends it with the help of an external agent. The dynamics between human and tool, subject and object, user and utilized becomes blurred due to the collaborative aspect of working with such a model. Even in the simple example laid out here, we can see the beginning of a challenge to the assumption 'I control technology—even as technology extends and forms my capabilities, its impact on the world is a reflection of my own agency.' I haven't even begun to describe the potential of these models to direct and be directed by the contours of our lives, yet we start to see the ways in which they can influence them.



As AI technology becomes increasingly integrated into our daily lives, we are at a crossroads. This note demonstrates how even seemingly simple AI products like GPT can significantly influence and direct our decisions. The ongoing evolution of AI, aimed at enhancing its persuasiveness, accessibility, and impact, underscores the urgency for a proactive approach in integrating AI into our personal and professional realms. We must strive to design these technologies to increase human agency and autonomy, ensuring AI serves as an empowering tool rather than a mechanism for external control, limiting individual freedoms. As we venture further into this AI-augmented era, it becomes imperative to remain vigilant and conscientious about the roles these technologies play in shaping our lives and society.