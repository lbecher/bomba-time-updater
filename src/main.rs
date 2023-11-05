use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        match update_system_time() {
            Ok(()) => {
                // Se a operação for bem-sucedida, saia do loop
                break;
            }
            Err(err) => {
                eprintln!("Erro ao obter a hora e data: {}", err);
                // Espere 5 segundos antes de tentar novamente
                sleep(Duration::from_secs(5));
            }
        }
    }

    Ok(())
}

fn update_system_time() -> Result<(), std::io::Error> {
    // Atualiza a hora do sistema
    let output = Command::new("ntpdate")
        .arg("pool.ntp.org")
        .output()?;

    // Se não der certo, retorna erro
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other, 
            String::from_utf8_lossy(&output.stderr),
        ));
    }

    Ok(())
}
