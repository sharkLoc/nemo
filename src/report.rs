use plotly::{
    Histogram, Layout, Plot, Scatter,
    common::{Font,Mode},
    layout::Axis,
    color::NamedColor,
    traces::table::{Cells, Header, Table, Fill}
};
use askama::Template;
use crate::error::NemoError;
use crate::process::Rec;
use crate::utils::file_writer;
use crate::cmd::VERSION;
use std::{collections::HashMap, vec};
use chrono::{DateTime,Local};

#[derive(Template)] 
#[template(path = "template.html", escape = "none")]
struct Report {
    version: String,
    report_time: String,
    basic_statistics: String,
    read_length_hist: String,
    per_read_gc: String,
    cumu_base: String,
    cmd: String,
}


pub fn summary(
    data: Rec,
    length_hash: HashMap<usize,usize>,
    gc_hash: HashMap<u64,u64>,
    html: &str,
    cmd_txt: String, 
) -> Result<(), NemoError> {
    
    let tb1_div = basic_statistics(data.clone())?;
    let (cumu_plot_div, length_plot_div) = length_plot(length_hash, data.bases)?;
    let gc_plot_div = gc_plot(gc_hash)?;

    let mut writer = file_writer(Some(html), 0u32)?;
    let now: DateTime<Local> = Local::now();
    let time_string = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let report = Report {
        version: String::from(VERSION),
        report_time: time_string,
        basic_statistics: tb1_div, 
        read_length_hist: length_plot_div, 
        per_read_gc: gc_plot_div,
        cumu_base: cumu_plot_div,
        cmd: cmd_txt,
    };
    writer.write_all(report.to_string().as_bytes())?;
    writer.flush()?;
    
    Ok(())
}


fn basic_statistics(
    data: Rec,
) -> Result<String, NemoError> {
    let header = Header::new(vec!["Measure","Value"])
        .align("left")
        .fill(Fill::new().color(NamedColor::SkyBlue))
        .font(Font::new().size(16));
    
    let value =  vec![ data.file_name,
        format!("{}",data.reads), format!("{}",data.bases), format!("{}",data.min_len), format!("{}",data.max_len), format!("{:.2}",data.average_len),
        format!("{:.2}",data.gc_content * 100.0), format!("{}",data.nt_a), format!("{}",data.nt_t), format!("{}",data.nt_g), format!("{}",data.nt_c),
        format!("{}",data.nt_n), format!("{}:({:.2}%)", data.less1k, data.less1k_r * 100.0), format!("{}:({:.2}%)", data.less2k, data.less2k_r * 100.0), 
        format!("{}:({:.2}%)", data.less5k, data.less5k_r * 100.0), format!("{}:({:.2}%)", data.less10k, data.less10k_r * 100.0),
        format!("{}:({:.2}%)", data.less20k, data.less20k_r * 100.0), format!("{}:({:.2}%)", data.less50k, data.less50k_r * 100.0),
    ]; 
    
    let cell = Cells::new(
        vec![
                    vec!["File name".to_string(),"Total reads count".to_string(),"Total bases count".to_string(),"Min read length".to_string(),"Max read length".to_string(),"Average read length".to_string(),
                    "GC Content(%)".to_string(),"Base A count".to_string(), "Base T count".to_string(), "Base G count".to_string(), "Base C count".to_string(), "Base N count".to_string(),"<1kb read info".to_string(),
                    "<2kb read info".to_string(), "<5kb read info".to_string(),"<10kb read info".to_string(),"<20kb read info".to_string(),"<50kb read info".to_string()],
                    value
                ]
            )
            .align("left")
            .fill(Fill::new().color(NamedColor::WhiteSmoke));

    
    let table = Table::new(header, cell);
    let mut plot = Plot::new();
    plot.add_trace(table);
    plot.set_layout(Layout::new()
        .title("<b>Sequencing summary</b>")
        .auto_size(false)
        .height(650)
        .width(600)
    );
    let tb1_div = plot.to_inline_html(Some("Basic_Statistics"));

    Ok(tb1_div)
}


fn length_plot(
    length_hash: HashMap<usize,usize>,  
    total_base: usize,
) -> Result<(String,String), NemoError> {
    let mut pairs : Vec<(&usize, &usize)> = length_hash.iter().collect();  
    pairs.sort_by_key(|x| x.0);  //  eq =>  tmp.sort_by(|x,y| x.0.cmp(y.0));
    
    // for cumulativa plot
    let mut vecx = vec![0];
    let mut vecy = vec![0.0];
    let mut cumulative_sum = 0.0;
    for (&x,&y) in pairs.iter() {
        vecx.push(x);
        cumulative_sum += y as f64 * x as f64;
        vecy.push( cumulative_sum / total_base as f64 * 100.);
    }

    let cumu = Scatter::new(vecx, vecy)
        .mode(Mode::Lines);
    let mut cumu_line = Plot::new();
    cumu_line.add_trace(cumu);
    cumu_line.set_layout(Layout::new()
        .title("<b>Cumulative fraction of bases</b>")
        .x_axis(Axis::new().title("Minimum read length(bp)"))
        .y_axis(Axis::new().title("Proportion(%)"))
        .auto_size(false)
        .width(800)
        .height(600)
    );
    let plot_div0 = cumu_line.to_inline_html(Some("Cumulative_Base"));

    // for length plot
    let mut datax = vec![];
    let mut datay = vec![];  
    for (&x,&y) in pairs.iter() {
        datax.push(x);
        datay.push(y);
    }
    
    let hist = Histogram::new_xy(datax.clone(), datay.clone());
    let mut plot = Plot::new();
    plot.add_trace(hist);
    plot.set_layout(Layout::new()
        .title("<b>Read length distribution</b>")
        .x_axis(Axis::new().title("Read length"))
        .y_axis(Axis::new().title("Read count"))
        .auto_size(false)
        .width(800)
        .height(600)
    );
    let plot_div = plot.to_inline_html(Some("Read_Length"));
    Ok((plot_div0, plot_div))
}

fn gc_plot(
    data:HashMap<u64,u64>
) -> Result<String, NemoError> {
    let total = data.values().sum::<u64>() as f64;
    let mut datax = vec![];
    let mut datay = vec![];
    for i in 0..=100 {
        let cnt = *data.get(&i).unwrap_or(&0);
        let ratio = (cnt as f64 * 10000.0 / total).round() / 100.0;
        datax.push(i);
        datay.push(ratio);
    }

    let line = Scatter::new(datax, datay).mode(Mode::Lines);
    let mut plot = Plot::new();
    plot.add_trace(line);
    plot.set_layout(Layout::new()
        .title("<b>Read GC content</b>")
        .y_axis(Axis::new().title("Proportion(%)"))
        .x_axis(Axis::new().title("Per-Read gc content(%)"))
        .auto_size(false)
        .width(800)
        .height(600)
    );
    let plot_div = plot.to_inline_html(Some("Read_GC_Content")); 

    Ok(plot_div)
}

