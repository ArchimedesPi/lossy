<pre>


         _             _            _           _      _        _   
        _\ \          /\ \         / /\        / /\   /\ \     /\_\ 
       /\__ \        /  \ \       / /  \      / /  \  \ \ \   / / / 
      / /_ \_\      / /\ \ \     / / /\ \__  / / /\ \__\ \ \_/ / /  
     / / /\/_/     / / /\ \ \   / / /\ \___\/ / /\ \___\\ \___/ /   
    / / /         / / /  \ \_\  \ \ \ \/___/\ \ \ \/___/ \ \ \_/    
   / / /         / / /   / / /   \ \ \       \ \ \        \ \ \     
  / / / ____    / / /   / / /_    \ \ \  _    \ \ \        \ \ \    
 / /_/_/ ___/\ / / /___/ / //_/\__/ / / /_/\__/ / /         \ \ \   
/_______/\__\// / /____\/ / \ \/___/ /  \ \/___/ /           \ \_\  
\_______\/    \/_________/   \_____\/    \_____\/             \/_/  
                                                                    


</pre>

## What's this?
An operating system written in [Rust](http://rust-lang.org),
aiming to do more than echoing the keyboard to the screen ;)

## What can it do right now?
Nothing. At all. (this is quite ironic, given the previous statement)

## What do you plan to do with it?
See [ROADMAP.md](https://github.com/ArchimedesPi/lossy/blob/master/ROADMAP.md)

## How can I help?
Find something to do (the [issues](https://github.com/ArchimedesPi/lossy/issues) will help with that),
fix it, and submit a PR. Oh, by the way, you might want to read about [hacking on Lossy](#hacking-on-the-code).

## Hacking on the code
You need Rust installed. Get it through [multirust](https://github.com/brson/multirust). After installation, run `multirust override nightly` in the project directory.
Then run `make runtime` to build libcore (with some patches applied).

Following this setup, you can simply execute `make run` to build and run in QEMU.
