use futures::channel::mpsc::{channel, Sender, Receiver};
use futures::StreamExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{AudioContext, AudioBuffer, AudioBufferSourceNode, GainNode};
use std::collections::HashMap;
use std::sync::Mutex as StdMutex;
use std::sync::Arc;

pub enum AudioCommand {
    PlaySound(String),
    PlayMusic(String),
    StopMusic,
    SetVolume(f32),
}

#[wasm_bindgen]
pub struct AudioSystem {
    context: AudioContext,
    sound_buffers: Arc<StdMutex<HashMap<String, AudioBuffer>>>,
    current_music: Arc<StdMutex<Option<AudioBufferSourceNode>>>,
    gain_node: GainNode,
    command_sender: Sender<AudioCommand>,
}

#[wasm_bindgen]
impl AudioSystem {
    pub async fn new() -> Result<AudioSystem, JsValue> {
        let context = web_sys::AudioContext::new()?;
        let gain_node = context.create_gain()?;
        gain_node.connect_with_audio_node(&context.destination())?;
        
        let (sender, receiver) = channel(32);
        
        let system = AudioSystem {
            context,
            sound_buffers: Arc::new(StdMutex::new(HashMap::new())),
            current_music: Arc::new(StdMutex::new(None)),
            gain_node,
            command_sender: sender,
        };
        
        // 启动音频命令处理循环
        system.start_audio_loop(receiver);
        
        Ok(system)
    }
    
    pub async fn load_sound(&self, name: &str, url: &str) -> Result<(), JsValue> {
        let array_buffer = self.fetch_audio_data(url).await?;
        let audio_buffer = self.decode_audio_data(array_buffer).await?;
        
        if let Ok(mut buffers) = self.sound_buffers.lock() {
            buffers.insert(name.to_string(), audio_buffer);
        }
        
        Ok(())
    }
    
    async fn fetch_audio_data(&self, url: &str) -> Result<js_sys::ArrayBuffer, JsValue> {
        let window = web_sys::window().unwrap();
        let resp_value = wasm_bindgen_futures::JsFuture::from(
            window.fetch_with_str(url)
        ).await?;
        
        let resp: web_sys::Response = resp_value.dyn_into()?;
        let array_buffer = wasm_bindgen_futures::JsFuture::from(resp.array_buffer()?).await?;
        
        Ok(array_buffer.dyn_into()?)
    }
    
    async fn decode_audio_data(&self, array_buffer: js_sys::ArrayBuffer) -> Result<AudioBuffer, JsValue> {
        let promise = self.context.decode_audio_data(&array_buffer)?;
        match wasm_bindgen_futures::JsFuture::from(promise).await {
            Ok(buffer) => Ok(buffer.dyn_into()?),
            Err(e) => {
                web_sys::console::error_1(&format!("Failed to decode audio: {:?}", e).into());
                Err(e)
            }
        }
    }
    
    pub fn play_sound(&self, name: &str) {
        let _ = self.command_sender.clone().try_send(AudioCommand::PlaySound(name.to_string()));
    }
    
    pub fn play_music(&self, name: &str) {
        let _ = self.command_sender.clone().try_send(AudioCommand::PlayMusic(name.to_string()));
    }
    
    pub fn stop_music(&self) {
        let _ = self.command_sender.clone().try_send(AudioCommand::StopMusic);
    }
    
    pub fn set_volume(&self, volume: f32) {
        let _ = self.command_sender.clone().try_send(AudioCommand::SetVolume(volume));
    }
    
    fn start_audio_loop(&self, mut receiver: Receiver<AudioCommand>) {
        let context = self.context.clone();
        let sound_buffers = Arc::clone(&self.sound_buffers);
        let current_music = Arc::clone(&self.current_music);
        let gain_node = self.gain_node.clone();
        
        wasm_bindgen_futures::spawn_local(async move {
            while let Some(command) = receiver.next().await {
                match command {
                    AudioCommand::PlaySound(name) => {
                        if let Ok(buffers) = sound_buffers.lock() {
                            if let Some(buffer) = buffers.get(&name) {
                                let source = context.create_buffer_source().unwrap();
                                source.set_buffer(Some(buffer));
                                source.connect_with_audio_node(&gain_node).unwrap();
                                source.start().unwrap();
                            }
                        }
                    }
                    AudioCommand::PlayMusic(name) => {
                        if let Ok(mut current) = current_music.lock() {
                            // 停止当前音乐
                            if let Some(source) = current.take() {
                                let _ = source.stop();
                            }
                            
                            // 播放新音乐
                            if let Ok(buffers) = sound_buffers.lock() {
                                if let Some(buffer) = buffers.get(&name) {
                                    let source = context.create_buffer_source().unwrap();
                                    source.set_buffer(Some(buffer));
                                    source.connect_with_audio_node(&gain_node).unwrap();
                                    source.set_loop(true);
                                    source.start().unwrap();
                                    *current = Some(source);
                                }
                            }
                        }
                    }
                    AudioCommand::StopMusic => {
                        if let Ok(mut current) = current_music.lock() {
                            if let Some(source) = current.take() {
                                let _ = source.stop();
                            }
                        }
                    }
                    AudioCommand::SetVolume(volume) => {
                        gain_node.gain().set_value(volume);
                    }
                }
            }
        });
    }
}