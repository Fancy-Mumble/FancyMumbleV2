import { FrontendSettings } from "../features/users/frontendSettings";
import { AudioInputSettings } from "../features/users/audioSettings";
import { LazyStore } from "@tauri-apps/plugin-store";

export const persistentStorage = new LazyStore(".settings.dat");

export async function persistFrontendSettings(frontendSettings: FrontendSettings) {
    await persistentStorage.set('frontendSettings', { ...frontendSettings });
    await persistentStorage.save();
}

export async function persistAudioSettings(audioSettins: AudioInputSettings) {
    console.log("Persisting audio settings: ", audioSettins);
    await persistentStorage.set('audioSettings', { ...audioSettins });
    await persistentStorage.save();
}