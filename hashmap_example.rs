// 一文读懂Rust宏（一） ---- 声明式宏 - 知乎
// https://zhuanlan.zhihu.com/p/620495523
/**
 * 声明式宏允许开发者以类似于函数的形式定义宏，并通过参数匹配和模板替换的方式来生成代码。基本语法如下:
macro_rules! macro_name {
    // 匹配模式和模板替换
    (pattern1) => { /* code1 */ };
    (pattern2) => { /* code2 */ };
    // 更多模式和模板替换
}
*/
use std::collections::HashMap;
/**
 * 这一段是匹配参数，属于最难理解的部分，$ ( xxx ),* 格式表明对参数匹配时可以对xxx部分进行0次或若干次匹配
 * 这里不用"*"，但用"+"表示至少需要匹配一次,后面的$(,)?可以匹配（0或1次）最后一次出现的逗号；
 * 这里*的作用类似于正则表示式的; 当然也可以换成 ? "0或1次 或者 + "至少1次" )
 * xxx参数用逗号进行分隔且最后一次匹配不能有逗号；
 * $key: expr => $val: expr 是参数的格式，expr代表匹配的类型，可以是任何有效的rust表达式
 */
#[macro_export] // 表示该宏导出可供其他包使用
macro_rules! hashmap { // 表示该宏的名字是 hashmap
    () => { std::collections::HashMap::new() }; // 匹配空map的情况

    ($ ($key: expr => $val: expr),+ $(,)?  ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $val);  )+ // $( map.insert($key, $val); )* 进行代码的执行，执行次数与上面参数匹配的次数相同，$key $val就是每次匹配到参数的值
             map
        }
    };
}

/// 测试代码
#[test]
pub fn test_hashmap() {
    let map1: HashMap<i32, &str> = hashmap!();
    let map2: HashMap<i32, &str> = hashmap!(1 => "one", 2=> "two", 3=> "three" );
    let map3: HashMap<i32, &str> = hashmap!(1 => "one", 2=> "two", 3=> "three", );
}
