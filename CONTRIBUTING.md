# Contributing to Quadify

*This is my first ever open-source project, so this file will change a lot. I also will be glad to learn
from other, more experienced people on how it's done (if there will be some)*

To start, Quadify is a minimal Bevy plugin, designed to provide minimal windowing/graphics/sound API's and more. 
It should be as plug-in as possible for new users to use (Like other bevy plugins), but some changes and limitations to the API
are acceptable (Say for rendering).
The goal is to limit the amount of dependencies the user has to compile/recompile each time a user builds a project with Quadify
(thus allowing bevy to be used even on poor hardware), but also reducing the output release binary size by a lot.

## What is this bizzare name?

Honestly, I don't know. It's like, this plugin *quadifies* your project, with bare-simple quads. So it's more simple.
I just like project names that end with "..fy".

## Macroquad or Miniquad

I've chose macroquad because:
- It's a very talented and known to me project
- Very lightweight to use (Recompiles are extremely fast)
- Very tiny footpring (In most personal cases, less than 500KB release)
- A lot of great abstractions over windowing, graphics, state, geometry batching etc.

The only problem with using macroquad with bevy, is that essentially there are 2 engines, with their 2 independent states.
Macroquad (and miniquad) heavily rely on global variables, while Bevy tries to be more strict; Unless there's a strong overhead 
of using both at the same time, it might be a great idea of using miniquad, quad-snd and so on directly.

I think I'm mentioning this, because I don't think this project should really be strongly bound to macroquad, since it's really
very well adapted for their own games. Bevy already has its own proper architecture, so the only reason I'm using macroquad is
that it already has a lot of useful features built-in, and the overhead is unnoticeble. Nevertherless, if you are experienced
in miniquad and low-level programming - yes, this project is for you.

*(This question is of course not permanent and discussable in the future)*

## How can I contribute?

### Issues or Pull Requests

If you have a proposal, problem or a bug - I would really appreaciate. Essentially most problems would be the problems of macroquad,
but there will be also integration bugs and API problems that are hard to debug and find (especially since we're talking about multi-platform support).

If you've just made some code changes and want to directly merge it (mostly something simple, fix) - you can.
For something more complex it's better to open an issue and discuss that.

### Tests

Writing tests for the plugin are also really appreciated. 

### Building plugins around the plugin

While it seems ironic, if you have some features in mind that are useful with Quadify (say, rendering) - it would be awesome
if you could build something around that.

### Documentation

Not even sure how it works in the cargo world, but would also really appreciate if you can help with documentation.

## Personal motivation

I've decided to make this plugin public, since I constantly see people complaining about huge bevy binary sizes. While not being a big
problem on its own - it's also painful to use it on poor hardware. In most cases I don't even need the reach feature-set bevy provides,
so it was really difficult for me to choose between macroquad and bevy.

I think even if not a big success, this project could motivate other more experienced developers create alternative low-level backends.
to the existing bevy ecosystem, creating even more alternatives for people to choose.
