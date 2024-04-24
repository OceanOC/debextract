use std::env;
use std::process::Command;

fn main() {
    println!("Debextract 0.1.0");
    let args: Vec<String> = env::args().collect();
    let ch = args[1].chars().next().unwrap();

    let mut debfile = format!("{}_{}_all.deb", args[1], args[2]);
    let mut url = format!("http://deb.debian.org/debian/pool/main/{}/{}/{}_{}_all.deb", ch, args[1], args[1], args[2]);

    // Architecture argument
    if args.len() >= 4 {
        url = format!("http://deb.debian.org/debian/pool/main/{}/{}/{}_{}_{}.deb", ch, args[1], args[1], args[2], args[3]);
        debfile = format!("{}_{}_{}.deb", args[1], args[2], args[3]);
    } else if args.len() == 0 || args[1] == "help" || args[1] == "--help" || args[1] == "-h" {
        println!("Usage: debextract [Package Name] [Version] [Architecture]")
    }

    println!("{}", url);
    //return;

    println!("Downloading from Debian source servers...");

    let output1 = Command::new("wget")
                     .arg(url)
                     .output()
                     .expect("failed to execute process");

    println!("{}", output1.status);
    println!("{}", String::from_utf8_lossy(&output1.stderr));
                 
    assert!(output1.status.success());

    println!("Decompressing deb file...");

    // .deb files use ar compression

    let output1 = Command::new("ar")
                     .arg("x")
                     .arg(debfile)
                     .output()
                     .expect("failed to execute process");

    println!("{}", output1.status);
    println!("{}", String::from_utf8_lossy(&output1.stderr));
                 
    assert!(output1.status.success());

    // .deb files usally contain 3 files
    //     + data.tar.xz - Contains the executable
    //     + control.tar.xz - Contains the metadata related to the file
    // Here we only need data.tar.xz

    println!("Decompressing data...");

    let output2 = Command::new("tar")
                     .arg("--extract")
                     .arg("-f")
                     .arg("data.tar.xz")
                     .arg("--xz")
                     .output()
                     .expect("failed to execute process");

    println!("{}", output2.status);
    println!("{}", String::from_utf8_lossy(&output2.stderr));

    assert!(output2.status.success());

    println!("Do you want to move the executable to '/usr/bin' and add it to PATH environment? (requires root) [Y/n]");

    let mut line = String::new();

    // This is required if we dont want a warning from the compiler
    #[allow(unused_variables)] let b1 = std::io::stdin().read_line(&mut line).unwrap();  #[warn(unused_variables)]

    if line.contains("y") || line.contains("Y"){
        let output3 = Command::new("mv")
                     .arg("usr")
                     .arg("/")
                     .output()
                     .expect("failed to execute process");

        println!("{}", output3.status);
        println!("{}", String::from_utf8_lossy(&output3.stderr));

        assert!(output3.status.success());

        println!("Cleaning up...");

        let output4 = Command::new("rm")
                     .arg("-rf")
                     .arg("control.tar.xz")
                     .arg("data.tar.xz")
                     .arg("debian-binary")
                     .arg(format!("{}", format!("{}_{}_{}.deb", args[1], args[2], args[3])))
                     .output()
                     .expect("failed to execute process");

        println!("{}", output4.status);
        println!("{}", String::from_utf8_lossy(&output4.stderr));
             
        assert!(output4.status.success());
    } else {
        println!("Cleaning up...");

        let output4 = Command::new("rm")
                     .arg("-rf")
                     .arg("control.tar.xz")
                     .arg("data.tar.xz")
                     .arg("debian-binary")
                     // debfile variable wasnt working so i had use this
                     .arg(format!("{}", format!("{}_{}_{}.deb", args[1], args[2], args[3])))
                     .output()
                     .expect("failed to execute process");

        println!("{}", output4.status);
        println!("{}", String::from_utf8_lossy(&output4.stderr));
             
        assert!(output4.status.success());
    }

    

}
