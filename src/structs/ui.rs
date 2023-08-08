//test
#[allow(dead_code)]
enum Input {
    UP,
    Left,
    Right,
    Down,
    Accept(String),
    Back(String)
}

#[allow(dead_code)]
impl Input{
    fn to_int(&self) -> i8{
        match self{
            Input::UP   =>  1,
            Input::Down => -1,
            _=> 0
        }
    }
}

#[allow(dead_code)]
struct State {
    selections:Vec<u8>,
    input: Input,
}
