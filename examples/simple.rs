// NOTE: This is quick demo example with using raw binding, it will be changed to safe wrapper

use piper_tts_rs_sys::{
    PIPER_DONE, piper_audio_chunk, piper_create, piper_default_synthesize_options, piper_free,
    piper_synthesize_next, piper_synthesize_start,
};
use std::ffi::CString;
use std::io::Write;
use std::mem::MaybeUninit;

fn main() {
    let mut file = std::fs::File::create("audio.raw").unwrap();

    let c_model_path = CString::new("model.onnx").unwrap();
    let c_model_config_path = CString::new("model.onnx.json").unwrap();
    let c_espeak_ng_data_path =
        CString::new("/path/to/espeak-ng-data").unwrap();
    let c_inference_text = CString::new(
        r#"Bugun osmon o‘zbek o‘zbek tiniq, shamol sokin. Ko‘chada hayot asta oqmoqda. Har kim o‘z ishida, o‘z niyatida. Vaqtni behuda ketkazmasdan, oldinga qarab yurish muhim.
    "#).unwrap();

    let synt = unsafe {
        piper_create(
            c_model_path.as_ptr(),
            c_model_config_path.as_ptr(),
            c_espeak_ng_data_path.as_ptr(),
        )
    };

    let options = unsafe { piper_default_synthesize_options(synt) };
    unsafe { piper_synthesize_start(synt, c_inference_text.as_ptr(), &options) };

    let mut chunk = unsafe { MaybeUninit::<piper_audio_chunk>::zeroed().assume_init() };
    while unsafe { piper_synthesize_next(synt, &mut chunk) } != PIPER_DONE as i32 {
        let audio_chunk = unsafe { std::mem::transmute::<*const f32, *const u8>(chunk.samples) };
        let audio_chunk_sz: usize = chunk.num_samples;
        let buffer =
            unsafe { std::slice::from_raw_parts(audio_chunk, audio_chunk_sz * size_of::<f32>()) };
        file.write(buffer)
            .expect("error writing pcm chunks to raw file");
    }

    unsafe { piper_free(synt) };

    println!("File created successfully!");
}
