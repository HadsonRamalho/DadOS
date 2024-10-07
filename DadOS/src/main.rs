#![no_std] // Não carrega a std
#![no_main] // Desativa todos os entry-points do Rust, incluindo a main

use::core::panic::PanicInfo;

// Função é chamada no panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

static HELLO: &[u8] = b"Hello World";

#[no_mangle] // Impede que o compilador reescreva o nome da função
            // (Permite que a função seja passada como parâmetro para o linker)
pub extern "C" fn _start() -> !{
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate(){
        unsafe{
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x40;
        }
    }

    loop{}
}

// Compilar com o target em 'x86_64-blog_os.json'
// Instalar bootimage pelo cargo
// Instalar QEMU

// cargo run vai executar o qemu