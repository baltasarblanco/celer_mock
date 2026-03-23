use nix::cmsg_space;
use nix::sys::socket::{recvmsg, ControlMessageOwned, MsgFlags};
use memmap2::Mmap;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::os::unix::net::UnixListener;
use std::fs::{remove_file, File};

fn main() {
    let socket_path = "/tmp/celer_bridge.sock";
    let _ = remove_file(socket_path); // Limpiar socket anterior si quedó sucio

    // 1. Abrir el puerto de escucha
    let listener = UnixListener::bind(socket_path).expect("Fallo al crear socket");
    println!("👁️‍🗨️ [CELER] Centinela activo. Escuchando en el socket temporal...");

    let (stream, _) = listener.accept().expect("Fallo al aceptar conexión de Aegis");
    println!("👁️‍🗨️ [CELER] Conexión entrante detectada. Extrayendo paquetes...");

    // 2. Preparar los buffers para atrapar la Magia Negra (SCM_RIGHTS)
    let mut iov_buf = [0u8; 4];
    let mut iov = [std::io::IoSliceMut::new(&mut iov_buf)];
    let mut cmsg_buffer = cmsg_space!([std::os::unix::io::RawFd; 1]);

    let msg = recvmsg::<()>(stream.as_raw_fd(), &mut iov, Some(&mut cmsg_buffer), MsgFlags::empty())
        .expect("Fallo al recibir el mensaje");

    // 3. Buscar el File Descriptor entre los mensajes de control del Kernel
    let mut received_fd = -1;
    for cmsg in msg.cmsgs() {
        if let ControlMessageOwned::ScmRights(fds) = cmsg {
            received_fd = fds[0];
            break;
        }
    }

    if received_fd == -1 {
        panic!("👁️‍🗨️ [CELER] Error: Aegis no envió el File Descriptor.");
    }

    // 4. Materializar el puente: Mapear la memoria a través del FD recibido
    let file = unsafe { File::from_raw_fd(received_fd) };
    let mmap = unsafe { Mmap::map(&file).expect("Fallo al mapear la memoria compartida") };

    // 5. Leer la mente de Aegis
    let raw_bytes = &mmap[..46]; // Longitud exacta de nuestro mensaje de prueba
    let texto = std::str::from_utf8(raw_bytes).unwrap();
    
    println!("👁️‍🗨️ [CELER] EXTRACCIÓN EXITOSA. Leyendo memoria de Aegis: '{}'", texto);
}