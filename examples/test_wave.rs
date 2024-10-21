use plotters::prelude::*;
use rand::{rngs::StdRng, SeedableRng};
use slot_algorithm::wave;

fn main() {
    let mut rng = StdRng::from_entropy();
    // 假设这里调用了 wave 模块生成波浪
    let a = wave::create_wave(0, 0, 10000, &mut rng);

    // 创建一个 800x600 的绘图区域
    let root_area = BitMapBackend::new("wave_output.png", (800, 600)).into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    // 使用生成的波浪数据，调整 Y 轴的最大值
    let y_max = a.iter().max().cloned().unwrap_or(1); // 获取波浪数据中的最大值，如果没有数据则默认是 1

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Wave Chart", ("sans-serif", 50))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..a.len(), 0..y_max) // X 轴是波浪数据的索引，Y 轴是波浪数据的值
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    // 将波浪数据绘制为折线图
    chart
        .draw_series(LineSeries::new(
            a.iter().enumerate().map(|(idx, &val)| (idx, val)), // 将波浪数据转换为 (index, value) 格式
            &BLUE,
        ))
        .unwrap()
        .label("Wave")
        .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &BLUE));

    // 显示图例
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();

    // 保存生成的图像
    println!("Wave chart has been saved as 'wave_output.png'");
}
