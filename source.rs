use std::env;
use std::fs ; 

fn main() {
 let args: Vec<String> = env::args().collect();

 let file: &String = &args[1];

 let folder: &String = &args[2];

 let pic01_ca: String = format!("{}{}", folder, "\\pic01_ca.dds".to_string()); println!("{:?}", fs::copy(file, pic01_ca));
 let pic02_ca: String = format!("{}{}", folder, "\\pic02_ca.dds".to_string()); println!("{:?}", fs::copy(file, pic02_ca));
 let pic03_ca: String = format!("{}{}", folder, "\\pic03_ca.dds".to_string()); println!("{:?}", fs::copy(file, pic03_ca));
 let pic04_ca: String = format!("{}{}", folder, "\\pic04_ca.dds".to_string()); println!("{:?}", fs::copy(file, pic04_ca));
 let pic05_ca: String = format!("{}{}", folder, "\\pic05_ca.dds".to_string()); println!("{:?}", fs::copy(file, pic05_ca));
 let pic06_nv: String = format!("{}{}", folder, "\\pic06_nv.dds".to_string()); println!("{:?}", fs::copy(file, pic06_nv));
 let pic07_nv: String = format!("{}{}", folder, "\\pic07_nv.dds".to_string()); println!("{:?}", fs::copy(file, pic07_nv));
 let pic08_nv: String = format!("{}{}", folder, "\\pic08_nv.dds".to_string()); println!("{:?}", fs::copy(file, pic08_nv));
 let pic09_az: String = format!("{}{}", folder, "\\pic09_az.dds".to_string()); println!("{:?}", fs::copy(file, pic09_az));
 let pic10_az: String = format!("{}{}", folder, "\\pic10_az.dds".to_string()); println!("{:?}", fs::copy(file, pic10_az));
 let pic11_az: String = format!("{}{}", folder, "\\pic11_az.dds".to_string()); println!("{:?}", fs::copy(file, pic11_az));
 let pic12_nm: String = format!("{}{}", folder, "\\pic12_nm.dds".to_string()); println!("{:?}", fs::copy(file, pic12_nm));
 let pic13_nm: String = format!("{}{}", folder, "\\pic13_nm.dds".to_string()); println!("{:?}", fs::copy(file, pic13_nm));
 let pic14_nm: String = format!("{}{}", folder, "\\pic14_nm.dds".to_string()); println!("{:?}", fs::copy(file, pic14_nm));
 let pic15_nm: String = format!("{}{}", folder, "\\pic15_nm.dds".to_string()); println!("{:?}", fs::copy(file, pic15_nm));
 let pic16_or: String = format!("{}{}", folder, "\\pic16_or.dds".to_string()); println!("{:?}", fs::copy(file, pic16_or));
 let pic17_or: String = format!("{}{}", folder, "\\pic17_or.dds".to_string()); println!("{:?}", fs::copy(file, pic17_or));
 let pic18_or: String = format!("{}{}", folder, "\\pic18_or.dds".to_string()); println!("{:?}", fs::copy(file, pic18_or));
 let pic19_wa: String = format!("{}{}", folder, "\\pic19_wa.dds".to_string()); println!("{:?}", fs::copy(file, pic19_wa));
 let pic20_wa: String = format!("{}{}", folder, "\\pic20_wa.dds".to_string()); println!("{:?}", fs::copy(file, pic20_wa));
 let pic21_wa: String = format!("{}{}", folder, "\\pic21_wa.dds".to_string()); println!("{:?}", fs::copy(file, pic21_wa));
 let pic22_wa: String = format!("{}{}", folder, "\\pic22_wa.dds".to_string()); println!("{:?}", fs::copy(file, pic22_wa));
 let pic23_wa: String = format!("{}{}", folder, "\\pic23_wa.dds".to_string()); println!("{:?}", fs::copy(file, pic23_wa));
 let pic24_ut: String = format!("{}{}", folder, "\\pic24_ut.dds".to_string()); println!("{:?}", fs::copy(file, pic24_ut));
 let pic25_ut: String = format!("{}{}", folder, "\\pic25_ut.dds".to_string()); println!("{:?}", fs::copy(file, pic25_ut));
 let pic26_ut: String = format!("{}{}", folder, "\\pic26_ut.dds".to_string()); println!("{:?}", fs::copy(file, pic26_ut));
 let pic27_ut: String = format!("{}{}", folder, "\\pic27_ut.dds".to_string()); println!("{:?}", fs::copy(file, pic27_ut));
 let pic28_ut: String = format!("{}{}", folder, "\\pic28_ut.dds".to_string()); println!("{:?}", fs::copy(file, pic28_ut));
 let pic29_or: String = format!("{}{}", folder, "\\pic29_or.dds".to_string()); println!("{:?}", fs::copy(file, pic29_or));
}//fn main() {