import { invoke } from "@tauri-apps/api";

export function openInBrowser(url: string) {
    invoke('open_browser', { url: url });
}