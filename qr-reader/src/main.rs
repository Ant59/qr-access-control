use opencv::{
    highgui::{named_window, WINDOW_AUTOSIZE, imshow, wait_key},
    core::{Mat, MatTraitManual},
    objdetect::{
        QRCodeDetector,
        QRCodeDetectorTrait,
    },
    videoio::{
        VideoCapture,
        VideoCaptureTrait,
        CAP_GSTREAMER,
    },
    Result,
    types::{VectorOfMat, VectorOfPoint, VectorOfString},
};
use std::{
    sync::mpsc::sync_channel,
    thread::spawn,
};

fn main() -> Result<()> {
    let window = "video capture";
    named_window(window, WINDOW_AUTOSIZE)?;

    let detector = QRCodeDetector::default()?;

    let mut source = VideoCapture::from_file("rtsp://admin:@192.168.0.164:554/h264Preview_01_main", CAP_GSTREAMER)?;

    let opened = VideoCapture::is_opened(&source)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    let (tx, rx) = sync_channel(1);
    spawn(move || {
        loop {
            let mut frame = Mat::default();
            source.read(&mut frame).unwrap();

            if frame.size().unwrap().width <= 0 {
                continue;
            }

            match tx.try_send(frame) {
                Ok(_) => continue,
                Err(_) => continue,
            }
        }
    });

    loop {
        let frame = rx.recv().unwrap();

        let mut rects = VectorOfPoint::default();
        let mut data = VectorOfString::default();
        let mut codes = VectorOfMat::default();
        detector.detect_and_decode_multi(&frame, &mut data, &mut rects, &mut codes)?;

        for value in &data {
            println!("{}", value);
        }

        imshow(window, &frame)?;

        let key = wait_key(10)?;
        if key > 0 && key != 255 && key != 235 {
            break;
        }
    }

    Ok(())
}
