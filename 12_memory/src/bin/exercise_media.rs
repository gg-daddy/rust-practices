// Problem 4: Fix the code by adding the type annotation

struct AudioSample;

struct ImageFile;

trait Media {}

impl Media for AudioSample {}
impl Media for ImageFile {}

fn main() {
    let audio_1 = AudioSample;
    let audio_2 = Box::new(AudioSample);

    let audio_3 = audio_1;
    let audio_4 = audio_2;

    let image_1 = Box::new(ImageFile);

    //注意下面的 Vec type annotation 使用的是 <> ，而不是 [] 。
    let media_collection: Vec<Box<dyn Media>> = vec![Box::new(audio_3), audio_4, image_1];
    // Fix this line
}
