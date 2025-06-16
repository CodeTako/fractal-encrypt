use fractal_key::FractalKey;
use fractal_stream::FractalStream;

mod complex;
mod fractal_key;
mod fractal_stream;

fn main() {
    let key = FractalKey::new();
    let mut stream = FractalStream::new(key);

    for i in 0..100 {
        println!("{}", stream.next());
    }
}
