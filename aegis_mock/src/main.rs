use nix::sys::memfd::{memfd_create, MemFdCreateFlag};
use nix::sys::socket::{sendmsg, ControlMessage, MsgFlags};
use nix::unistd::ftruncate;
use memmap2::MmapMut;
use std::ffi::CString;
use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("[AEGIS] Iniciando secuencia de arranque...");

    // 1. Pedir RAM anónima al Kernel de Linux
    let name = CString::new("celer_bridge").unwrap();
    let fd = memfd_create(&name, MemFdCreateFlag::MFD_CLOEXEC | MemFdCreateFlag::MFD_ALLOW_SEALING)
        .expect("Fallo al crear memfd");

    // 2. Darle tamaño a la memoria (4096 bytes = 1 página de memoria)
    ftruncate(&fd, 4096).expect("Fallo al dimensionar la memoria");
    // 3. Mapear la memoria a nuestro proceso y escribir el mensaje
    let mut mmap = unsafe { MmapMut::map_mut(fd.as_raw_fd()).expect("Fallo al mapear") };
    let mensaje = b"HOLA CELER: EL PUENTE ESTA ABIERTO Y ASEGURADO";
    mmap[..mensaje.len()].copy_from_slice(mensaje);
    println!("[AEGIS] Memoria escrita. Datos sellados en RAM.");

    // Esperar a que Celer levante el socket...
    sleep(Duration::from_secs(2));

    // 4. Conectarse al socket de Celer
    let stream = UnixStream::connect("/tmp/celer_bridge.sock")
        .expect("Celer no está escuchando en el socket");

    // 5. La Magia Negra: Enviar el File Descriptor (FD) usando SCM_RIGHTS
    let iov = [std::io::IoSlice::new(b"ping")]; // Linux exige enviar al menos 1 byte normal
    let cmsgs = [ControlMessage::ScmRights(&[fd.as_raw_fd()])];
    
    sendmsg::<()>(stream.as_raw_fd(), &iov, &cmsgs, MsgFlags::empty(), None)
        .expect("Fallo al inyectar el FD en el socket");

    println!("[AEGIS] File Descriptor transferido. Manteniendo proceso vivo...");
    sleep(Duration::from_secs(10)); 
}