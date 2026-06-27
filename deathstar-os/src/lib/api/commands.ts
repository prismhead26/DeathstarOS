import { invoke } from '@tauri-apps/api/core';

export interface AudioState {
  muted: boolean;
  volume: number;
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

// Audio Commands
export async function toggleAudioMute(): Promise<boolean> {
  return await invoke<boolean>('toggle_audio_mute');
}

export async function getAudioMuteState(): Promise<boolean> {
  return await invoke<boolean>('get_audio_mute_state');
}

export async function getAudioVolume(): Promise<number> {
  return await invoke<number>('get_audio_volume');
}

export async function setAudioVolume(volume: number): Promise<void> {
  return await invoke('set_audio_volume', { volume });
}

export async function getAudioState(): Promise<AudioState> {
  return await invoke<AudioState>('get_audio_state');
}

// Microphone Commands
export async function toggleMicrophone(): Promise<boolean> {
  return await invoke<boolean>('toggle_microphone');
}

export async function getMicrophoneMuteState(): Promise<boolean> {
  return await invoke<boolean>('get_microphone_mute_state');
}

export async function getMicrophoneVolume(): Promise<number> {
  return await invoke<number>('get_microphone_volume');
}

// Camera Commands
export async function getCameraStatus(): Promise<boolean> {
  return await invoke<boolean>('get_camera_status');
}

export async function isCameraInUse(): Promise<boolean> {
  return await invoke<boolean>('is_camera_in_use');
}

export async function toggleCamera(): Promise<boolean> {
  return await invoke<boolean>('toggle_camera');
}

export async function getCameraState(): Promise<CameraState> {
  return await invoke<CameraState>('get_camera_state');
}

export async function openCameraSettings(): Promise<void> {
  return await invoke('open_camera_settings');
}

// Network Commands
export async function toggleWifi(): Promise<boolean> {
  return await invoke<boolean>('toggle_wifi');
}

export async function getWifiState(): Promise<boolean> {
  return await invoke<boolean>('get_wifi_state');
}

export async function toggleBluetooth(): Promise<boolean> {
  return await invoke<boolean>('toggle_bluetooth');
}

export async function getBluetoothState(): Promise<boolean> {
  return await invoke<boolean>('get_bluetooth_state');
}

export async function getNetworkState(): Promise<NetworkState> {
  return await invoke<NetworkState>('get_network_state');
}

// Brightness Commands
export async function getBrightness(): Promise<number> {
  return await invoke<number>('get_brightness');
}

export async function setBrightness(level: number): Promise<void> {
  return await invoke('set_brightness', { level });
}

export async function openAccessibilitySettings(): Promise<void> {
  return await invoke('open_accessibility_settings');
}

// Focus/DND Commands
export async function toggleDoNotDisturb(): Promise<boolean> {
  return await invoke<boolean>('toggle_do_not_disturb');
}

export async function getDoNotDisturbState(): Promise<boolean> {
  return await invoke<boolean>('get_do_not_disturb_state');
}

export async function openFocusSettings(): Promise<void> {
  return await invoke('open_focus_settings');
}
