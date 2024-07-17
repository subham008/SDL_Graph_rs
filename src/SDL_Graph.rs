use sdl2::{pixels::Color, render::{RenderTarget, Texture},};


pub struct  GraphDataset{
    data:Vec<(i32 , i32)>,
    bar_color:Color,
    bar_color_array:Vec<Color>,
}


impl GraphDataset {
    fn get_max_x(self)->i32{
            let mut max =  self.data[0].0; // set initial value
            // now running loop to find max number 
            for i in self.data{
                 let x = i.0;
                 if x>max {
                     max = x.clone();
                 }
            }
            return max;
    }

    fn get_min_x(self)->i32{
        let mut min =  self.data[0].0; // set initial value
        // now running loop to find max number 
        for i in self.data{
             let x = i.0;
             if x<min {
                 min = x.clone();
             }
        }
        return min;
     }

     fn get_max_y(self)->i32{
        let mut max =  self.data[0].1; // set initial value
        // now running loop to find max number 
        for i in self.data{
             let x = i.1;
             if x>max {
                 max = x.clone();
             }
        }
        return max;
    }
    fn get_min_y(self)->i32{
        let mut min =  self.data[0].1; // set initial value
        // now running loop to find max number 
        for i in self.data{
             let x = i.1;
             if x<min {
                 min = x.clone();
             }
        }
        return min;
     }
 


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


impl Graph<'r> {
    fn get_max_x(self)->i32{
         let  fdata_set:&GraphDataset = &self.dataset_array[0];
         let max= fdata_set.get_max_x().clone();
         
         return  max;
    }

    fn get_min_x(self)->i32{
        let min = 89;
        return  min;
    }

    fn get_max_y(self)->i32{
        let max = 89;
         return  max;
    }

    fn get_min_y(self)->i32{
        let min = 89;
         return  min;
    }
}
