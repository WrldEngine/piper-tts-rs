use hound::WavSpec;
use piper_tts_rs::PiperSession;

#[test]
fn test_piper_headers() {
    let mut empty_buff = Vec::new();
    let expected_spec = WavSpec {
        channels: 1,
        sample_rate: 22050,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let session = PiperSession::new(
        "/home/serverboy/piper_model/model.onnx".to_string(),
        "/home/serverboy/piper_model/model.onnx.json".to_string(),
        None,
    )
    .unwrap();

    let text = "salom ozbekiston!";
    session
        .generate_wav(&mut empty_buff, text.to_string())
        .expect("failed to generate audio");

    let reader = hound::WavReader::new(empty_buff.as_slice()).unwrap();
    assert_eq!(reader.spec(), expected_spec);
}
