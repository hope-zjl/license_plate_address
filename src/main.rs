use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize)]
struct City {
    code: String,
    city: String,
    province: String,
}

const JSON_DATA: &str = include_str!("./data/json.json");

fn main() {
    println!("==============================");
    println!("=========*欢迎使用*===========");
    println!("==============================");
    println!("");
    println!("示例:京A    如需结束请输入 exit");
    loop {
        println!("输入车牌...");
        let mut area = String::new();
        std::io::stdin().read_line(&mut area).unwrap();
        let cities: Vec<City> = from_str(JSON_DATA).unwrap();
        if area.trim().to_lowercase() == "exit" {
            break;
        }
        match cities
            .iter()
            .position(|x| x.code.to_lowercase() == area.trim().to_lowercase())
        {
            Some(index) => {
                let cities_list = &cities[index];
                println!(
                    "{} 属于{}, {}",
                    cities_list.code, cities_list.city, cities_list.province
                );
            }
            None => println!("无"),
        }
    }

    println!("请按下回车键关闭窗口...");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
}
// rustc main.rs -o helloworld.exe
