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

## Roadmap
- [x] - Build system
- [x] - Bootable do-nothing system
- [x] - Video buffer driver
- [x] - Page frame allocator
- [x] - Interrupts
- [ ] - Keyboard driver
- [ ] - Allocator
- [ ] - Proper stack/heap
- [ ] - Switching out of BIOS video

## How can I help?
Find something to do (the [issues](https://github.com/ArchimedesPi/lossy/issues) will help with that),
fix it, and submit a PR. Oh, by the way, you might want to read about [hacking on Lossy](#hacking-on-the-code).

## Hacking on the code
You need the following dependencies:
- `grub-mkrescue`
- `xorriso`
- `mtools`
- `nasm`
- qemu
- Rust

Get Rust through [multirust](https://github.com/brson/multirust). After installation, run `multirust override nightly` in the project directory.
Run `make patch_libcore` to patch the nofp patch into libcore.
Then run `make runtime` to build libcore.

Following this setup, you can simply execute `make run` to build and run in QEMU.
