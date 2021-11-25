use std::io;

const MAGIC_NUMBERS: [i32; 15] = [10690, 12251, 17649, 24816, 33360, 35944, 36412, 42041, 42635, 44011, 53799, 56181, 58536, 59222, 61041];

fn salt(a: i32, b: i32, c: i32) -> i32 {
    let mut a = a;
    for i in 0..8 {
        let t = (b >> i) & 1;
        if t+((a - t) & !1) == a {
            a = (a - t) >> 1;
        } else {
            a = ((c - t) ^ a) >> 1;
        }
    }
    a
}

fn gen_password(origin: String, number: i32) -> String{
    let mut hash = number;
    for i in origin.chars().rev() {
        // dbg!(&i);
        hash = salt(hash, i as i32, 0x105C3);
    };
    // return "".to_string();
    let mut n1 = 0;
    while salt(salt(hash, n1 & 0xFF, 0x105C3), n1 >> 8, 0x105C3) != 0xA5B6 {
        n1 = n1 + 1;
        if n1 >= 0xFFFF {
            eprintln!("找不到对应注册码");
            return "".to_string();
        }
    }

    let n1: i32 = (((n1 + 0x72FA) & 0xFFFF) as f64 * 99999.0 / 0xFFFF as f64) as i32;
    let n1str: String = "0000".to_owned() + &n1.to_string();
    let n1str = (&n1str[n1str.len()-5..]).to_string();

    let temp = n1str[0..n1str.len()-3].to_string() + &n1str[n1str.len()-2..] + &n1str[n1str.len() - 3..n1str.len() - 2];
    let temp = temp.parse::<i32>().unwrap();

    let temp = ((temp as f64 / 99999.0) * 0xFFFF as f64) as i32 + 1;
    let mut temp = salt(salt(0, temp & 0xFF, 0x1064B), temp >> 8, 0x1064B);

    for i in origin.chars().rev() {
        temp = salt(temp, i as i32, 0x1064B);
    }

    let mut n2 = 0;

    while salt(salt(temp, n2 & 0xFF, 0x1064B), n2 >> 8, 0x1064B) != 0xA5B6 {
        n2 = n2 + 1;
        if n2 >= 0xFFFF {
            eprintln!("找不到对应注册码");
            return "".to_string();
        }
    }
    let n2: i32 = (((n2 & 0xFFFF) as f64) * 99999.0 / 0xFFFF as f64) as i32;
    let n2str = "0000".to_string() + &*n2.to_string();
    let n2str = &n2str[n2str.len()-5..];
    let n1str = &*n1str;
    let pass = format!("{}{}{}{}-{}{}{}-{}{}{}",
        &n2str.chars().nth(3).expect("索引错误"),
        &n1str.chars().nth(3).expect("索引错误"),
        &n1str.chars().nth(1).expect("索引错误"),
        &n1str.chars().nth(0).expect("索引错误"),

        &n2str.chars().nth(4).expect("索引错误"),
        &n1str.chars().nth(2).expect("索引错误"),
        &n2str.chars().nth(0).expect("索引错误"),

        &n2str.chars().nth(2).expect("索引错误"),
        &n1str.chars().nth(4).expect("索引错误"),
        &n2str.chars().nth(1).expect("索引错误")
    );
    format!("{}::1", &pass[0..12])
}

fn main() {
    println!("======= MMA 12.x Keygen by lihe07 =======");
    println!("说明: ");
    println!("本注册机适用于 Mathematica 数学分析软件的 12.x 版本");
    println!("在12.3 12.2 12.1版本上测试成功");
    println!("仅限用于逆向分析学习使用 **不得用于商业或者非法用途**");
    println!("请在下面输入你的Math ID: ");
    let mut math_id = String::new();
    io::stdin().read_line(&mut math_id).expect("Unable to readline!");
    math_id = math_id.trim().to_string();
    let activation_key = "1234-4321-123456";
    println!("\n\n === 计算结果 ===");
    println!("激活码: {}", activation_key);
    println!("密码(请从下列中选择任意一个): ");
    for number in MAGIC_NUMBERS {
        println!("{}", gen_password((&math_id).to_string() + "$1&" + &activation_key, number));
    };
    println!("\n按任意键退出...");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Unable to readline!");
}