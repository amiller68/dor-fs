## This is you:
- scared
- alone with your thoughts of an awesome Fullstack Web App that you want to develop a proof of concept (PoC) for. It's gonna curate and serve jokes about fruit, which is very exciting and innovative!
- yearning for an easy way to prototype, deploy, and iterate on your idea

You go to reach for the stack that has served you time and time again throughout the years:
- NextJs
- Typescript
- Postgres + ORM
- Vercel

You quiver as you look at your tools in front of you. You know success is around the corner, but your hands freeze as dread fills every fiber of your being. All you gotta do now is suck it up and:
- Create a yarn package that you know is going to have undiagnosable conflicts and behavior
- Setup your typescript project, making random choices about your config in order to please the transpiler
- Get your hands dirty writing starter react components. Where did you put those examples?
- Figure out which of the billion SQL ORMs is a good fit for the Vercel SDK your backend has to use
- Wait where'd you put that article on setting up Vercel Postgres? You know it's somewhere in your notes...
- Write five different files for each Database model you want to represent. Oh damn, how do you version control migrations with this all-in-one ORM you chose?
- Sacrifice a couple of goats in order to ward off the possibility that your backend requires anything more complex than a REST API
- Finally ready to deploy! But, oh wait, there's a build error on Vercel, time to figure out what's wrong with your node version
- Cool! The build logs are full of hard-to-understand warnings and errors that will probably be important later, but the site up!
- Now try not to think about the prospect of migrating to literally any other platform
- Never mind, relax, it will be fiiiine
- Just kidding, the pain remains deep, deep in your heart

Luckily for you, I swoop down from the trees, crushing your laptop between my heavily booted feet. Where did I come from? How are there suddenly a bunch of trees inside of your apartment? Why am I dressed like a cross between He-Man and a Sys Admin? How long have you left that Trader Joe's frozen Tikka Masala in the oven while stressing about your awesome Fullstack Web App for curating and serving fruit jokes? It smells like it's burning. Reading your mind I exclaim, "None, of that matters now! I am here to save you from yourself!"

## But first, how did we get here?

Let's back up a little bit -- so you had a great idea for a Fullstack Web App for curating and serving fruit jokes, awesome! You know you have the skills to build it, and you're pretty certain on the feature set. The important decisions you have to make is how exactly your going to deliver a PoC product efficiently, without sacrificing development experience later on as you continue to build on your original idea. You probably have some of the following goals in mind when it comes to doing so:
- Be concise: you want to get your idea off of the ground quickly. Getting a first implementation working should not require much code or boilerplate. Ideally it should not take longer than one stint working on the idea to get a prototype working.
- Be efficient: it should be easy to develop, test, and deploy your project without much context switching. It should be an easy choice to stay in the terminal, with a handful of well integrated tools guiding the way. Code interfaces should be interchangeable and expose common sense APIs.
- Be agile: you're building a PoC, yes, but maybe that idea turns out to be big! You want it to be easy to keep developing your ideas and shipping features to users.

You probably reached for the following tools because you thought using said components in your stack would enable your PoC features while helping achieve development goals:
- JavaScript / Typescript: its easy to prototype with JavaScript right? And if you need to annotate with types then you'll just sprinkle on some Typescript. This should help you be concise, and maybe allow you to experiment easily.
- NextJs + Vercel: Everyone seems to be using NextJs for Fullstack applications, and you get deployments for free with Vercel. This will definitely help you achieve efficiency when it comes to deployment.
- ORM: You decided you need a database, and the Vercel requires you to use an ORM with their Postgres product. That might be helpful anyway -- abstraction is always helpful, right?
And for the most part, depending on your level of experience and objectives, these tools will serve you well. Using this stack is a 100% viable development path, followed by lots of hobbyist and professional teams for delivering features and content to users. But maybe this all might just be either out-of-scope or otherwise not in line with your priorities -- ship code, ship it fast, ship it easy. If you go down this road, you might find it hard to:
- Be concise: you've just committed yourself writing configs for NextJs, Typescript, and your ORM, defining model boilerplate for managing your Postgres data, and writing React components. That's a solid chunk of complexity to start out with.
- Be efficient: How many tools do you think you are going to be using for package management, building, Postgres administration, and testing? I also wonder how long you're going to have to spend learning new SDKs to implement Postgres.
- Be agile: Every new Frontend feature requires an entire React component, or chunk of ORM boilerplate. This will eventually slow you down as your components get more complex!
It would be great if you had tools that would help you reap the benefits of project organization, database management, type safety, and bespoke Frontend development while avoiding some these headaches. It would be great if you knew about a simple stack that was a little bit more integrated and easy to work with. Oh, if only!

## Introducing the stack: Rust, Sqlx, HTMX, and Shuttle.rs

"What do you mean," you say gazing up into my magnificently bronzed face "where did you come from?!"

"Never mind that! What's important now is that I share the good news"

You know that Rust and its package manager Cargo are great tools for writing a type-safe, well versioned packages that are readily portable to a variety of platforms and architectures. You know it would be great if you could incorporate it into your stack and reap the benefits of its built-in formatter, compile-time checking, and testing kit in order to speed up your development process. 

"But, Rust is just for backend, right?" you ask, wondering if I just used telepathy to share that information. "I'm trying to quickly ship a web application, how could Rust help me?"

Rust alone won't work for your project. You still require:
- a strong database solution
- a means to ship a web app to a browser
- a place to quickly deliver your PoC to users
Ideally you want all of these tools to both play nicely with each other and help you stay concise, efficient, and agile. That's where the following stack comes in:
- [Sqlx](https://github.com/launchbadge/sqlx/tree/main) is both a a CLI (Command Line Interface) for managing various SQL databases and library for running type-safe and sanitized queries in Rust. It has version controlled migrations information that helps keep your schemas organized and easy to manage, which helps tremendously when you are going to production with your idea. it also has a ton of macros that help you cut down on boilerplate and focus on creating features. It does make you write SQL, but this ends up saving you a lot of unhelpful abstraction and boilerplate.
- [HTMX](https://htmx.org/) is a hypermedia library that gives you access to css, ajax, and server events within decorated Html. Paired with a templating library, HTMX is a powerful tool for consuming hypermedia APIs for serving modern web applications.
- [Shuttle.rs](https://www.shuttle.rs/) is a platform for quickly shipping web apps written in Rust. A simple deployment consisting of a container and a connection to a shared Postgres cluster are provisioned for your project. All you need to do is wrap one of your favorite Rust web frameworks in one of Shuttle's runtimes and you can easily ship code to users.

You scratch your head and look at me. "That's great and all, but how does this help me?"

That's a great question! These tools are great for a few reasons, but mainly they are better suited for helping you achieve those guiding objectives you came up with earlier:
- Cargo is gonna help organize all your tools, linting, checking, testing, and package management. This doesn't sound like a lot now, but it makes a big difference as projects grow in complexity. This will help you be efficient!
- Sqlx is going to help you implement features on top of your database while helping both keep migrations explicit and version controlled as well as avoid weighty abstractions you'd have to write with an ORM. This will help you be concise!
- HTMX is going to give you a tool to tightly bind your Backend logic to your Frontend development. This cuts down on context switching, boilerplate from writing React components, and provides great Frontend performance! This will help you be both concise and efficient!
- Shuttle.rs is going to help us ship features quick while taking advantage of Rust and its many benefits. Their runtimes work out-of-the-box with your framework of choice, or you could choose to write a custom service to enhance portability later on. This will help you be agile, and will set you up to be portable to more mature infrastructure when the time comes.
By adopting this approach, I think you'll find that lots of decisions become a lot easier, especially as your project grows, and more people start to contribute to it.

"Hmm. But I think I'm going to need to write and ship JavaScript to the browser for some features I'm considering. I don't think this approach will enable that!"

Hold on, hold on. You're just writing a PoC right? There's probably not a good reason to be shipping JavaScript to the browser for a quite a while, and you could get decent performance out of just a hypermedia API. That being said, you still have a variety of options later on if you want to implement client-side rendered content:
- Ship static vanilla JavaScript direct to the browser. You can still write and ship static JavaScript with HTMX! You could even include it in your templates!
- You can still [use a Frontend framework like NextJS](https://joshmo.hashnode.dev/deploying-a-nextjs-front-end-with-a-rust-api-in-one-go) in this stack, so that option always remains open -- but only after it is strictly required!
- Or, better yet, you could adopt a Rust-based Frontend framework like [Leptos](https://leptos.dev/) and ship well-tested and performant WASM directly from your Web App!
You shouldn't make the mistake of picking a tool *just* because you *think* it will help you later on -- you get a lot more benefit and options from choosing stacks that power the *requirements* of your application rather than the *nice-to-haves*. It would be *nice to have* a well-used Frontend framework like NextJs for writing complicated Frontend logic, but, as I said before, there are negative consequences associated with *starting* with NextJs, and you probably don't need the fancy Frontend logic neither.

A smile starts to spread across your face. "Huh, yeah I guess that makes sense, I'm gonna look into this!"

"I'm glad! Now it's time for me to depart, and share the good news with others throughout the world," I yell, bounding across your desk and disappearing back up into the trees that start to fade from view.

"Wait, what about my laptop?!"

A faint breeze sounds through the air as the last leaves depart from view. It sounds:

" ... not ... my     problem ...."

As the final echoes of my departing words fade away, leaving behind a silence that's only punctuated by the distant hum of the world outside, you're left in a moment of contemplation. The ideas that I've shared with you about using Rust, Sqlx, HTMX, and Shuttle.rs aren't just technical choices; they represent a new way of thinking, a paradigm shift in how you approach web app development.

You ponder over your crushed laptop, now a casualty in the quest for knowledge, and realize that it's not just about the tools you use, but about how you use them. It's about choosing a path that aligns with your goals, your vision, and the journey you want to embark on. Yes, your familiar tools have been your companions through many projects, but the world of technology is ever-evolving, always presenting new opportunities to learn, grow, and innovate.

You make a mental note to get a new laptop, and more importantly, to explore the vibrant communities around these new tools. As you sketch out your next steps, a sense of excitement washes over you. Yes, the path is uncharted, but it's the challenges and learning that make the journey worthwhile.

## Wrapping Up

As you reflect on this narrative and the journey it outlines, remember that the world of technology is vast and filled with endless possibilities. Whether you're a seasoned developer or just starting out, there's always something new to learn, a new perspective to consider, and a new challenge to conquer. The use of hypermedia APIs to serve Frontend content is an exciting new area of Web Development, and ties in neatly with other work in bringing type-safety and performance to the browser with WASM. It's time to jump in!

I hope you enjoyed my silly narrative about prototyping Web Apps in Rust. If you want to learn more, feel free to check out my [Fullstack Web App template for curating and serving jokes about fruit](https://github.com/amiller68/fruit-jokes). It should have everything you need to start writing extensible and performant prototypes for your best ideas! Good luck and happy coding!