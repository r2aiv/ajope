fn main() 
{
    println!("GovnoSoft Ajope Enterprise Pro 2023");

    let src_str = "rakom ded ebet kobylu";
    let mut proc_vec = Vec::new();
    let mut dst_str = String::new();

    for val in src_str.chars(){proc_vec.push(val);}
    proc_vec.reverse();
    for val in proc_vec {dst_str.push(val);}

    println!("Source: {}", src_str);
    println!("Dest:   {}", dst_str);

    println!("CTRL-C for exit");
    
    loop{};
}
