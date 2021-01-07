use neon::prelude::*;
use std::{cell::RefCell, fs::File, io::BufReader, sync::mpsc, thread, time::Duration};

#[derive(Clone)]
enum ControlEvent {
    Play,
    Pause,
    Stop,
    Volume(f32),
    Empty,
}

pub struct Player {
    control_tx: mpsc::Sender<ControlEvent>,
    info_rx: mpsc::Receiver<bool>,
}

impl Finalize for Player {}

impl Player {
    #[inline]
    fn new() -> Self {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let _ = rodio::Sink::try_new(&handle).unwrap();
        let (control_tx, _) = mpsc::channel();
        let (_, info_rx) = mpsc::channel();
        Self {
            control_tx,
            info_rx,
        }
    }

    #[inline]
    fn load(&mut self, url: String) -> bool {
        if let Ok(file) = File::open(url) {
            if let Ok(source) = rodio::Decoder::new(BufReader::new(file)) {
                self.stop();

                let (control_tx, control_rx) = mpsc::channel();
                let (info_tx, info_rx) = mpsc::channel();

                thread::spawn(move || {
                    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
                    let sink = rodio::Sink::try_new(&handle).unwrap();
                    sink.append(source);
                    let _ = info_tx.send(true);
                    loop {
                        match control_rx.recv() {
                            Ok(ControlEvent::Play) => sink.play(),
                            Ok(ControlEvent::Pause) => sink.pause(),
                            Ok(ControlEvent::Volume(level)) => sink.set_volume(level),
                            Ok(ControlEvent::Empty) => {
                                let _ = info_tx.send(sink.empty());
                            }
                            _ => {
                                drop(sink);
                                break;
                            }
                        }
                    }
                });

                self.control_tx = control_tx;
                self.info_rx = info_rx;
                let _ = self.info_rx.recv();
                return true;
            }
        }
        false
    }

    #[inline]
    fn play(&mut self) {
        let _ = self.control_tx.send(ControlEvent::Play);
    }

    #[inline]
    fn pause(&mut self) {
        let _ = self.control_tx.send(ControlEvent::Pause);
    }

    #[inline]
    fn stop(&mut self) {
        let _ = self.control_tx.send(ControlEvent::Stop);
    }

    #[inline]
    fn set_volume(&self, level: f32) {
        let _ = self.control_tx.send(ControlEvent::Volume(level));
    }

    #[inline]
    fn empty(&self) -> bool {
        if let Ok(_) = self.control_tx.send(ControlEvent::Empty) {
            if let Ok(res) = self.info_rx.recv_timeout(Duration::from_millis(128)) {
                return res;
            }
        }
        true
    }
}

pub fn player_new(mut cx: FunctionContext) -> JsResult<JsValue> {
    let player = RefCell::new(Player::new());

    Ok(cx.boxed(player).upcast())
}

pub fn player_load(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let player = cx.argument::<JsBox<RefCell<Player>>>(0)?;
    let url = cx.argument::<JsString>(1)?.value(&mut cx);
    let res = player.borrow_mut().load(url);

    Ok(cx.boolean(res))
}

pub fn player_play(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let player = cx.argument::<JsBox<RefCell<Player>>>(0)?;
    let mut player = player.borrow_mut();

    let res = match player.empty() {
        false => {
            player.play();
            true
        }
        _ => false,
    };

    Ok(cx.boolean(res))
}

pub fn player_pause(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let player = cx.argument::<JsBox<RefCell<Player>>>(0)?;
    player.borrow_mut().pause();

    Ok(cx.undefined())
}

pub fn player_stop(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let player = cx.argument::<JsBox<RefCell<Player>>>(0)?;
    player.borrow_mut().stop();

    Ok(cx.undefined())
}
pub fn player_volume(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let player = cx.argument::<JsBox<RefCell<Player>>>(0)?;
    let level = cx.argument::<JsNumber>(1)?.value(&mut cx) / 100.0;
    player.borrow().set_volume(level as f32);

    Ok(cx.undefined())
}

pub fn player_empty(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let player = cx.argument::<JsBox<RefCell<Player>>>(0)?;
    let res = player.borrow().empty();

    Ok(cx.boolean(res))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("playerEmpty", player_empty)?;
    cx.export_function("playerLoad", player_load)?;
    cx.export_function("playerNew", player_new)?;
    cx.export_function("playerPause", player_pause)?;
    cx.export_function("playerPlay", player_play)?;
    cx.export_function("playerVolume", player_volume)?;
    cx.export_function("playerStop", player_stop)?;

    Ok(())
}
