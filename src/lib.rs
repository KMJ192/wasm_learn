mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Array;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-js-snake-game!");
}

//#[wasm_bindgen] : javascript에 Binding 제공함

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Vector{
    pub x : f64,
    pub y : f64,
}

#[wasm_bindgen]
impl Vector{
    //constructor : 바인딩 되는 함수가 실제로 javascript에서 연산자 호출로 변환해야 함을 나타내는데 사용 됨.
    #[wasm_bindgen(constructor)]
    pub fn new(x : f64, y : f64) -> Vector {
        Vector{ x, y }
    }

    pub fn subtract(&self, other : &Vector) -> Vector{
        Vector::new(self.x - other.x, self.y - other.y)
    }
    pub fn scale_by(&self, number : f64) -> Vector{
        Vector::new(self.x * number, self.y * number)
    }
}

#[wasm_bindgen]
pub struct Game{
    pub width : i32,
    pub height : i32,
    pub speed  :f64,
    pub score : i32,
    pub direction : Vector,
    pub food : Vector,
    snake : Vec<Vector>
}

#[wasm_bindgen]
impl Game{
    #[wasm_bindgen(constructor)]
    pub fn new(width : i32, height : i32, speed : f64, snake_length : i32, direction : Vector) -> Game{
        let head_x = {f64::from(width) / 2_f64}.round() - 0.5;
        let head_y = {f64::from(height) / 2_f64}.round() - 0.5;
        let head = Vector::new(head_x, head_y);
        let tailtip = head.subtract(&direction.scale_by(f64::from(snake_length)));
        let snake = vec![tailtip, head];
        //TODO : place food in random cell;
        let food = Vector::new(0.5, 0.5);

        Game{
            width : width,
            height : height,
            speed : speed,
            snake : snake,
            direction : direction,
            food : food,
            score : 0,
        }
    }
    
    //javascript를 반환하는 public method
    pub fn get_snake(&self) -> Array {
        self.snake.clone().into_iter().map(JsValue::from).collect()
    }
    
}