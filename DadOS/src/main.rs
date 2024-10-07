#![no_std] // Não carrega a std
#![no_main] // Desativa todos os entry-points do Rust, incluindo a main

use::core::panic::PanicInfo;

// Função é chamada no panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

#[no_mangle] // Impede que o compilador reescreva o nome da função
            // (Permite que a função seja passada como parâmetro para o linker)
pub extern "C" fn _start() -> !{
    loop{}
}

// Compilar com o target em 'x86_64-blog_os.json'