/**
 * @file Api.tsx
 * @description Api interface for the Tauri app to communicate with the Rust backend and the React frontend.
 * @author
 * @version 0.0.1
 * @date 2021-06-01
 *
 */

import { invoke } from "@tauri-apps/api/tauri";

export default class Api {
  static async requestOpenFileDialog(): Promise<string> {
    return await invoke("requestOpenFileDialog");
  }

  static async requestImportXml(xml_path: string): Promise<void> {
    await invoke("requestImportXml", { xml_path });
  }

  static async requestExportXml(): Promise<void> {
    await invoke("requestExportXml");
  }

  static async requestReload(): Promise<void> {
    await invoke("requestReload");
  }

  static async requestSetting(): Promise<void> {
    await invoke("requestSetting");
  }

  static async requestClose(): Promise<void> {
    await invoke("requestClose");
  }

  static async requestMinimize(): Promise<void> {
    await invoke("requestMinimize");
  }

  static async requestMaximize(): Promise<void> {
    await invoke("requestMaximize");
  }

  static async requestToggleMaximize(): Promise<void> {
    await invoke("requestToggleMaximize");
  }

  static async requestToggleSetting(): Promise<void> {
    await invoke("requestToggleSetting");
  }

  static async requestToggleSidebar(): Promise<void> {
    await invoke("requestToggleSidebar");
  }

  static async requestToggleContent(): Promise<void> {
    await invoke("requestToggleContent");
  }
}
