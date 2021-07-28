use plotters::prelude::*;

pub fn plot(data: Vec<u32>, data2: Vec<u32>, data3: Vec<u32>) {
    let root_area = SVGBackend::new("graph.svg", (1380, 720)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let (upper, lower) = root_area.split_vertically(360);
    let max = *data.iter().max().unwrap().max(data2.iter().max().unwrap()) + 1;

    let mut ctx = ChartBuilder::on(&upper)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Population", ("sans-serif", 30))
        .build_cartesian_2d(0..data.len() as u32 + 1, 0..max)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new((0..).zip(data.iter().map(|x| *x)), &BLUE))
        .unwrap()
        .label("Organism")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    ctx.draw_series(LineSeries::new((0..).zip(data2.iter().map(|x| *x)), &RED))
        .unwrap()
        .label("Predator")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();
    let (lower_left, lower_right) = lower.split_horizontally(420);
    let (lrl, lrr) = lower_right.split_horizontally(420);
    let all_datas = [data, data2, data3];
    let drawing_areas = [lower_left, lrl, lrr];
    for (drawing_area, idx) in drawing_areas.iter().zip(0..) {
        let d = &all_datas[idx];
        let col = [BLUE, RED, GREEN][idx];
        let mut cc = ChartBuilder::on(&drawing_area)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption(["Organism", "Predator", "Food"][idx], ("sans-serif", 15))
            .build_cartesian_2d(0..d.len() as u32 + 1, 0..*d.iter().max().unwrap())
            .unwrap();
        cc.configure_mesh().draw().unwrap();
        cc.draw_series(
            AreaSeries::new((0..).zip(d.iter().map(|x| *x)), 0, &col.mix(0.2)).border_style(&col),
        )
        .unwrap();
    }
}
