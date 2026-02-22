import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export default function MusicWidget() {
    const [track, setTrack] = useState<{ title: string; artist: string; is_playing: boolean } | null>(null);

    useEffect(() => {
        // Get initial track state on mount
        invoke<{ title: string; artist: string; is_playing: boolean } | null>("get_current_track").then(setTrack);

        const unlisten = listen<{ title: string; artist: string; is_playing: boolean }>("media-change", (event) => {
            setTrack(event.payload);
        });
        return () => {
            unlisten.then((f) => f());
        };
    }, []);

    return (
        <div className="music-bar" data-tauri-drag-region>
            <div className={`music-visualizer ${track?.is_playing ? 'playing' : ''}`} data-tauri-drag-region>
                <div className="vis-bar"></div>
                <div className="vis-bar"></div>
                <div className="vis-bar"></div>
                <div className="vis-bar"></div>
            </div>
            {track ? (
                <div className="music-info" data-tauri-drag-region>
                    <span className="music-title" data-tauri-drag-region>{track.title}</span>
                    {track.artist && (
                        <>
                            <span className="music-sep" data-tauri-drag-region>·</span>
                            <span className="music-artist" data-tauri-drag-region>{track.artist}</span>
                        </>
                    )}
                </div>
            ) : (
                <span className="music-idle" data-tauri-drag-region>No music playing</span>
            )}
        </div>
    );
}
