use sdl2::{pixels::Color, render::Texture,};


pub struct  GraphDataset{
    data:Vec<Vec<i32>>,
    bar_color:Color,
    bar_color_array:Vec<Color>
}


pub struct  Graph<'r>{
     dataset_array:Vec<GraphDataset>,
     text_color:Color,
     background_color:Color,
     show_graph_lines:bool,
     flags:u8,
     x_title:String,
     y_title:String,
     width:i32,
     height:i32,
     x_title_texture:Texture<'r>,
     y_title_texture:Texture<'r>,
}

