import { invoke } from '@tauri-apps/api/core';

export interface AudioState {
  muted: boolean;
  volume: number; // 0–100
}

export interface NetworkState {
  wifi_enabled: boolean;
  bluetooth_enabled: boolean;
}

export interface CameraState {
  enabled: boolean;
  in_use: boolean;
  used_by: string | null;
}

// Audio
export async function toggleAudioMute(): Promise<boolean> {
  return await invoke<boolean>('toggle_audio_mute');
}

export async function setAudioVolume(volume: number): Promise<void> {
  return await invoke('set_audio_volume', { volume });
}

export async function getAudioState(): Promise<AudioState> {
  return await invoke<AudioState>('get_audio_state');
}

// Microphone
export async function toggleMicrophone(): Promise<boolean> {
  return await invoke<boolean>('toggle_microphone');
}

export async function getMicrophoneMuteState(): Promise<boolean> {
  return await invoke<boolean>('get_microphone_mute_state');
}

export async function getMicrophoneVolume(): Promise<number> {
  return await invoke<number>('get_microphone_volume');
}

// Camera
export async function toggleCamera(): Promise<boolean> {
  return await invoke<boolean>('toggle_camera');
}

export async function getCameraState(): Promise<CameraState> {
  return await invoke<CameraState>('get_camera_state');
}

export async function openCameraSettings(): Promise<void> {
  return await invoke('open_camera_settings');
}

// Network
export async function toggleWifi(): Promise<boolean> {
  return await invoke<boolean>('toggle_wifi');
}

export async function toggleBluetooth(): Promise<boolean> {
  return await invoke<boolean>('toggle_bluetooth');
}

export async function getNetworkState(): Promise<NetworkState> {
  return await invoke<NetworkState>('get_network_state');
}

// Brightness
export async function getBrightness(): Promise<number> {
  return await invoke<number>('get_brightness');
}

export async function setBrightness(level: number): Promise<void> {
  return await invoke('set_brightness', { level });
}

// Focus / Do Not Disturb
export async function toggleDoNotDisturb(): Promise<boolean> {
  return await invoke<boolean>('toggle_do_not_disturb');
}

export async function getDoNotDisturbState(): Promise<boolean> {
  return await invoke<boolean>('get_do_not_disturb_state');
}

export async function openFocusSettings(): Promise<void> {
  return await invoke('open_focus_settings');
}
