// use rand::Rng;
use std::{io::prelude::*, fs, fs::*, env};
// fn main() {
//   let mut buf = String::new();
//   println!("Name me: ");
//   io::stdin().read_line(&mut buf).expect("Failed to read line");
//   println!("Hello, world! My name is {}", buf);
// }

// fn main() -> std::io::Result<()> {
//     let args: Vec<String> = env::args().collect();
//     let flag = &args[1];
//     let destination_path = &args[2];
//     println!("{}", destination_path);
//     let mut file = File::create("autostart")?;
//     // println!("{:?}", args);
//     if flag == "-s" || flag == "--show" || flag == "show" {
//     file.write_all(b"@lxpanel --profile LXDE-pi
// @pcmanfm --desktop --profile LXDE-pi
// @xscreensaver -no-splash")?;
//       println!("Show");
//     }else if flag == "-h" || flag == "--hide" || flag == "hide"{
//       file.write_all(b"#@lxpanel --profile LXDE-pi
// @pcmanfm --desktop --profile LXDE-pi
// @xscreensaver -no-splash")?;
//       println!("Hide");
//     }
// //     let mut file = File::open("./autostart")?;
// //     let mut contents = String::new();
// //     file.read_to_string(&mut contents)?;
// //     assert_eq!(contents, "@lxpanel --profile LXDE-pi
// // @pcmanfm --desktop --profile LXDE-pi
// // @xscreensaver -no-splash");
//     // println!("{:?}", contents);
//     std::fs::copy("autostart", destination_path)?;
//     Ok(())
// }

/*
Show/hide task bar
*/
// fn main() -> std::io::Result<()> {
//   let args: Vec<String> = env::args().collect();
//   let flag = &args[1];
//   {
//     let mut file = File::create("temp")?;
//     if flag == "-s" || flag == "--show" || flag == "show"{
//       file.write_all(b"@lxpanel --profile LXDE-pi
// @pcmanfm --desktop --profile LXDE-pi
// @xscreensaver -no-splash")?;
//       println!("Show task bar");
//     }else if flag == "-h" || flag == "--hide" || flag == "hide"{
//       file.write_all(b"#@lxpanel --profile LXDE-pi
// @pcmanfm --desktop --profile LXDE-pi
// @xscreensaver -no-splash")?;
//       println!("Hide tassk bar");
//     }
//   }
//   let destination_path = &args[2];
//   fs::copy("temp", destination_path)?;
//   fs::remove_file("temp")?;
//   Ok(())
// }

/*
Create desktop shortcut
*/
// fn main() -> std::io::Result<()> {
//   let args: Vec<String> = env::args().collect();
//   let flag = &args[1];
//   let s = String::from("Hi");
//   // Create a temporary file.
//   let temp_directory = env::temp_dir();
//   let temp_file = temp_directory.join("file");
//   // Open a file in write-only (ignoring errors).
//   // This creates the file if it does not exist (and empty the file if it exists).
//   let mut file = File::create(temp_file).unwrap();
//   // Write a &str in the file (ignoring the result).
//   writeln!(&mut file, "Hello World!").unwrap();

//   // Write a byte string.
//   file.write(b"Bytes\n").unwrap();
//   fs::copy(&temp_file, "file")?;
//   Ok(())
// }
fn main() -> std::io::Result<()> {
  // let s = String::from("Hi");
  // // Create a temporary file.
  //   let temp_directory = env::temp_dir();
  //   let temp_file = temp_directory.join("file");

  //   // Open a file in write-only (ignoring errors).
  //   // This creates the file if it does not exist (and empty the file if it exists).
  //   let mut file = File::create(temp_file).unwrap();

  //   // Write a &str in the file (ignoring the result).
  //   writeln!(&mut file, "Hello World!").unwrap();

  //   // Write a byte string.
  //   file.write(b"Bytes\n").unwrap();

  //   let path = env::current_dir()?;
  //   let b = path.parent().unwrap();
  //   println!("The current directory is {}\n{}", path.display(), b.display());
  //   fs::copy(temp_directory.join("file"), path.join("file"))?;
  //   Ok(())
  let path = env::current_dir()?;
  let file = path.join("chienowa.desktop");
  fs::copy(&file, "/home/pi/.config/autostart/")?;
  fs::copy(&file, "/usr/share/applications/")?;
  println!("Created Hand Washing Machine Desktop");
  Ok(())
}