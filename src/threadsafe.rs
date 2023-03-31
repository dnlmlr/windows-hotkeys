use std::{
    marker::PhantomData,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{spawn, JoinHandle},
};

use crate::{
    error::HkError,
    keys::{ModKey, VKey},
    singlethreaded, HotkeyId, HotkeyManagerImpl, InterruptHandle,
};

struct Hotkey<T: 'static> {
    key: VKey,
    key_modifiers: Vec<ModKey>,
    extra_keys: Vec<VKey>,
    callback: Box<dyn Fn() -> T + Send + 'static>,
}

enum HkMsg<T: 'static> {
    Register(Sender<Result<HotkeyId, HkError>>, Hotkey<T>),
    HandleHotkey(Sender<Option<T>>),
    Unregister(Sender<Result<(), HkError>>, HotkeyId),
    UnregisterAll(Sender<Result<(), HkError>>),
    EventLoop(Sender<()>),
    InterruptHandle(Sender<InterruptHandle>),
    Exit(Sender<()>),
}

pub struct HotkeyManager<T: 'static> {
    _phantom: PhantomData<T>,
    snd: Sender<HkMsg<T>>,
    backend_handle: Option<JoinHandle<()>>,
}

struct TSHotkeyManagerBackend<T: 'static> {
    hkm: singlethreaded::HotkeyManager<T>,
    rec: Receiver<HkMsg<T>>,
}

impl<T> TSHotkeyManagerBackend<T> {
    /// Create a new HotkeyManager instance. To work around the same-thread limitation of the
    /// windows event API, this will launch a new background thread to handle hotkey interactions.
    ///
    fn new(rec: Receiver<HkMsg<T>>) -> Self {
        let hkm = singlethreaded::HotkeyManager::new();
        Self { hkm, rec }
    }

    fn backend_loop(&mut self) {
        while let Ok(msg) = self.rec.recv() {
            match msg {
                HkMsg::Register(chan_ret, hk) => {
                    let ret_val = self.hkm.register_extrakeys(
                        hk.key,
                        &hk.key_modifiers,
                        &hk.extra_keys,
                        hk.callback,
                    );
                    chan_ret.send(ret_val).unwrap();
                }
                HkMsg::HandleHotkey(chan_ret) => {
                    let ret_val = self.hkm.handle_hotkey();
                    chan_ret.send(ret_val).unwrap();
                }
                HkMsg::Unregister(chan_ret, hkid) => {
                    let ret_val = self.hkm.unregister(hkid);
                    chan_ret.send(ret_val).unwrap();
                }
                HkMsg::UnregisterAll(chan_ret) => {
                    let ret_val = self.hkm.unregister_all();
                    chan_ret.send(ret_val).unwrap();
                }
                HkMsg::EventLoop(chan_ret) => {
                    self.hkm.event_loop();
                    chan_ret.send(()).unwrap();
                }
                HkMsg::InterruptHandle(chan_ret) => {
                    let ret_val = self.hkm.interrupt_handle();
                    chan_ret.send(ret_val).unwrap();
                }
                HkMsg::Exit(chan_ret) => {
                    chan_ret.send(()).unwrap();
                    return;
                }
            }
        }
    }
}

impl<T: 'static + Send> HotkeyManagerImpl<T> for HotkeyManager<T> {
    fn new() -> Self {
        let (snd, rec) = channel();
        let backend_handle = spawn(move || {
            let mut backend = TSHotkeyManagerBackend::<T>::new(rec);
            backend.backend_loop();
        });

        Self {
            _phantom: PhantomData::default(),
            snd,
            backend_handle: Some(backend_handle),
        }
    }

    fn register(
        &mut self,
        key: VKey,
        key_modifiers: &[ModKey],
        callback: impl Fn() -> T + Send + 'static,
    ) -> Result<HotkeyId, HkError> {
        self.register_extrakeys(key, key_modifiers, &[], callback)
    }

    fn register_extrakeys(
        &mut self,
        key: VKey,
        key_modifiers: &[ModKey],
        extra_keys: &[VKey],
        callback: impl Fn() -> T + Send + 'static,
    ) -> Result<HotkeyId, HkError> {
        let ret_ch = channel();
        let hk = Hotkey {
            key,
            key_modifiers: key_modifiers.to_vec(),
            extra_keys: extra_keys.to_vec(),
            callback: Box::new(callback),
        };
        self.snd.send(HkMsg::Register(ret_ch.0, hk)).unwrap();
        ret_ch.1.recv().unwrap()
    }

    fn unregister(&mut self, id: HotkeyId) -> Result<(), HkError> {
        let ret_ch = channel();
        self.snd.send(HkMsg::Unregister(ret_ch.0, id)).unwrap();
        ret_ch.1.recv().unwrap()
    }

    fn unregister_all(&mut self) -> Result<(), HkError> {
        let ret_ch = channel();
        self.snd.send(HkMsg::UnregisterAll(ret_ch.0)).unwrap();
        ret_ch.1.recv().unwrap()
    }

    fn handle_hotkey(&self) -> Option<T> {
        let ret_ch = channel();
        self.snd.send(HkMsg::HandleHotkey(ret_ch.0)).unwrap();
        ret_ch.1.recv().unwrap()
    }

    fn event_loop(&self) {
        let ret_ch = channel();
        self.snd.send(HkMsg::EventLoop(ret_ch.0)).unwrap();
        ret_ch.1.recv().unwrap()
    }

    fn interrupt_handle(&self) -> InterruptHandle {
        let ret_ch = channel();
        self.snd.send(HkMsg::InterruptHandle(ret_ch.0)).unwrap();
        ret_ch.1.recv().unwrap()
    }
}

impl<T> Drop for HotkeyManager<T> {
    fn drop(&mut self) {
        let ret_ch = channel();
        self.snd.send(HkMsg::Exit(ret_ch.0)).unwrap();
        ret_ch.1.recv().unwrap();
        self.backend_handle.take().unwrap().join().unwrap();
    }
}
