extern crate rand;
use crate::rand::Rng;

use macroquad::prelude::*;


const  RIGHT : i8 = 1;
const  LEFT : i8 = -1;
const  UP : i8 = -1;
const  DOWN : i8 = 1;
const GREEN1: Color = color_u8!(88,143,61,255);
const GREEN2: Color = color_u8!(170,191,64,255);
const TILE_SIZE : f32 = 32.0;


struct Entity {
    pos : Vec2,
    texture: Texture2D,
}

fn rand_float(start: i8 , end : i8) -> f32 {
    rand::thread_rng().gen_range(start..end as i8) as f32
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let window : Vec2 = vec2(17.0,15.0);

    let mut snake = Entity { pos : vec2(10.0,10.0) , texture : Texture2D::from_file_with_format(include_bytes!("../assets/snake.png"),Some(ImageFormat::Png))};
    let mut size: u32 = 0;
    let mut prev_poses: Vec<Vec2> = Vec::new();


    let mut target = Entity { pos : vec2(rand_float(0,window.x as i8),rand_float(0,window.y as i8)), texture : Texture2D::from_file_with_format(include_bytes!("../assets/target.png"),Some(ImageFormat::Png))};
    

    
    let mut dir : (i8 , i8) = (0 , 0);
    let mut last_dir : (i8 , i8) = (1 , 0);

    let mut last_update : f64 = 0.0;
    let update_speed: f64 = 0.4;
    let faster_update_speed: f64 = 0.25;

    let mut game_over = false;

    unsafe {
        get_internal_gl().quad_context.show_mouse(false);
    }
    request_new_screen_size(window.x * TILE_SIZE, window.y * TILE_SIZE);

    loop {
        clear_background(BLUE);

        if snake.pos.x < 0.0 || snake.pos.x >= window.x || snake.pos.y < 0.0 || snake.pos.y >= window.y { game_over = true; }
        for i in &prev_poses { if snake.pos == *i { game_over = true; break; } }
        if game_over { 
            unsafe {
                get_internal_gl().quad_context.quit();
            }
        }


        if dir == (0,0) {
            if (is_key_down(KeyCode::D) || is_key_down(KeyCode::Right))     && last_dir.0 != LEFT { dir.0 = RIGHT; }
            else if (is_key_down(KeyCode::A) || is_key_down(KeyCode::Left)) && last_dir.0 != RIGHT  { dir.0 = LEFT;}
            else if (is_key_down(KeyCode::W) || is_key_down(KeyCode::Up))   && last_dir.1 != DOWN  { dir.1 =  UP;}
            else if (is_key_down(KeyCode::S) || is_key_down(KeyCode::Down)) && last_dir.1 != UP {    dir.1 =  DOWN;}
        }
        if (get_time() - last_update > update_speed) || (dir == last_dir && get_time() - last_update > faster_update_speed)  {
            last_update = get_time();

            if size > 0 {
                prev_poses.push(vec2(snake.pos.x,snake.pos.y));
                if prev_poses.len() as u32 > size {
                    prev_poses.remove(0);
                }  
            }

            let used_dir: (i8 , i8) = if dir != (0 , 0) { dir } else { last_dir };
            snake.pos.x += used_dir.0 as f32;
            snake.pos.y += used_dir.1 as f32;
            last_dir = used_dir;
            dir = (0 , 0);

            if target.pos == snake.pos {
                target.pos = vec2(rand_float(0,window.x as i8),rand_float(0,window.y as i8));
                size += 1;
            }
        }


        //drawing
        for x in 0..(window.x as i8) {
            for y in 0..(window.y as i8) {
                let color = if (x + y) % 2 != 0 { GREEN1 }  else { GREEN2 } ;               
                draw_rectangle(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, TILE_SIZE, TILE_SIZE, color);
            }
        }
        draw_texture(target.texture,target.pos.x * TILE_SIZE, target.pos.y  * TILE_SIZE, WHITE);
        for i in &prev_poses {
            draw_texture(snake.texture,i.x * TILE_SIZE, i.y  * TILE_SIZE, WHITE);
        }
        draw_texture(snake.texture,snake.pos.x * TILE_SIZE, snake.pos.y  * TILE_SIZE, WHITE);

        next_frame().await
           

    }
}
