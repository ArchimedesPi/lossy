ARCH = 'i686-elf'
kernfile = 'lossy.bin'

task :default => [:compile, :link, :emulate]

task :compile do |t|
  asm('boot/boot.s')
  cpp('init/main.cpp')
  cpp('video/bios_term.cpp')
end

task :link do |t|
  linker({:files => '**/*.o', :output => kernfile})
end

task :emulate do |t|
  sh "qemu-system-i386 -kernel #{kernfile}"
end

def asm(file)
  sh "#{ARCH}-as #{file} -o #{file.split('.').first + '.o'}"
end

def cpp(file)
  sh "#{ARCH}-g++ -c #{file} -o #{file.split('.').first + '.o'} -ffreestanding -O2 -Wall -Wextra -I."
end

def linker(opt)
  sh "#{ARCH}-gcc -T linker.ld -o #{opt[:output]} -ffreestanding -O2 -nostdlib #{opt[:files]} -lgcc"
end