///////////////////////////////////////////////

// port scanner
mod port_sniffer;
use std::net::IpAddr;

// generating ip
use rand::prelude::*;

// ping
extern crate ping;
use std::time::Duration;
use rand::random;

// rtsp connection
use tokio::net::TcpStream;
use url::Url;

// window
extern crate winapi;

// misc
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::um::wincon::SetConsoleTitleW;
use std::io::Write;
use std::fs::OpenOptions;
use std::io;
/////////////////////////////////////////////
//

#[tokio::main]
async fn main() {
    let new_title = "rtsp@cam:~$: nico.dev";
    let new_title_wide: Vec<u16> = OsStr::new(new_title).encode_wide().chain(once(0)).collect();
    unsafe {
        SetConsoleTitleW(new_title_wide.as_ptr());
    }

    println!("rtsp@cam:~$ searching online ip...");

    ///////////////////////////////////////////////////////////////////////
    loop{
        let mut rng = rand::thread_rng();
        let n1= rng.gen_range(128..191);
        let n2= rng.gen_range(0..255);
        let h1= rng.gen_range(0..255);
        let h2= rng.gen_range(1..254);
    
        let ip_address = format!("{}.{}.{}.{}", n1, n2, h1, h2);

        //////////////////////////////////////////////////////////////
        let addr = ip_address.parse().unwrap();
        let timeout = Duration::from_millis(200);
    
        match ping::ping(addr, Some(timeout), Some(166), Some(3) ,Some(5), Some(&random())){
            Ok(_) => {
                println!("rtsp@cam:~$ ip: {} is online", ip_address); 
                
                //////////////////////////////////////////////////////
                let addr = ip_address;
                let port = 554;
                match TcpStream::connect((addr.clone(), port)).await {
                    Ok(_) => {
                        println!("rtsp@cam:~$ port {} is open", port);
                        io::stdout().flush().unwrap();
                        let url_string = format!("rtsp://admin:admin@{}:554", addr);
                        let url = match Url::parse(&url_string) {
                            Ok(url) => url,
                            Err(e) => {
                                eprintln!("error parsing URL: {}", e);
                                println!("");
                                return;
                            }
                        };
                    
                        let addr = match url.socket_addrs(|| Some(554)) {
                            Ok(addrs) => addrs.into_iter().next(),
                            Err(e) => {
                                eprintln!("error converting url to socketaddr: {}", e);
                                println!("");
                                return;
                            }
                        };
                    
                        match TcpStream::connect(addr.unwrap()).await {
                            Ok(_) => {
                                println!("rtsp@cam:~$ {} is open", url_string);
                                
                                let file_name = "vulnerable ips.txt";
                    
                                let mut file = match OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .open(file_name) {
                                    Ok(file) => file,
                                    Err(e) => {
                                        eprintln!("rtsp@cam:~$ error opening/creating file: {}", e);
                                        println!("");
                                        return;
                                    }
                                };
                                
                                let content = url_string;
                                
                                // Escribir el contenido en el archivo
                                if let Err(e) = writeln!(file, "{}", content) {
                                    eprintln!("rtsp@cam:~$ error appending to txt file: {}", e);
                                    println!("");
                                } else {
                                    println!("rtsp@cam:~$ content appended to txt file");
                                    println!("");
                                }
                            }
                    
                            Err(e) => {
                                eprintln!("rtsp@cam:~$ {} is closed {}", url_string, e);
                                println!("");
                            }
                        }
                    }
                    
                    Err(_) => {
                        println!("rtsp@cam:~$ port 554 closed, returning to ip generator");
                        println!("");
                    }
                }
                /////////////////////////////////////////////////////
            }
            Err(_) => {generate_ip();}
        }
        ////////////////////////////////////////////////////////
    }
    ////////////////////////////////////////////////////////////////////////
}

// generating a random ip, valid or not
fn generate_ip(){

}