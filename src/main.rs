// 使用pub use 重导出后就可以在顶级引用了
use art::mix;
use art::PrimaryColor;
use art::SecondaryColor;

// 否则 需要如下引用
// use art::kinds::PrimaryColor;
// use art::kinds::SecondaryColor;
// use art::utils::mix;

fn main() {
    // green + blue 混合成青色 cyan
    let green = PrimaryColor::Green;
    let blue = PrimaryColor::Blue;
    let cyan: SecondaryColor = mix(green, blue);
    println!("merge: {:?}", cyan);
}

// --- 关于工作空间

// 如果你选择向 crates.io发布工作空间中的 crate，每一个工作空间中的 crate 需要单独发布。
// cargo publish 命令并没有 --all 或者 -p 参数，所以必须进入每一个 crate 的目录并运行
// cargo publish 来发布工作空间中的每一个 crate。
