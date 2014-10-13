architecture = "i386"
executable_filetype = "elf"
ld = "#{architecture}-#{executable_filetype}-ld"
rustc = "rustc"
qemu = "qemu-system-#{architecture}"

image_file = "lossy.img"


task :default => [:build, :image, :run]

task :build do
end

task :image => [:build] do
end

task :run => [:image] do
  sh "#{qemu} #{image_file}"
end

task :print_config do
  puts "architecture: #{architecture}"
  puts "executable_filetype: #{executable_filetype}"
  puts "ld: #{ld}"
  puts "rustc: #{rustc}"
  puts "qemu: #{qemu}"
end
