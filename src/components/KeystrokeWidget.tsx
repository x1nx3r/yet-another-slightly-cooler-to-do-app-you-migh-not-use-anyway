import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

interface Keystroke {
    id: number;
    key: string;
    timestamp: number;
}

export default function KeystrokeWidget() {
    const [keys, setKeys] = useState<Keystroke[]>([]);

    useEffect(() => {
        let counter = 0;
        const unlisten = listen<string>("keystroke-event", (event) => {
            const newStroke: Keystroke = {
                id: counter++,
                key: event.payload,
                timestamp: Date.now(),
            };
            setKeys((prev) => [...prev, newStroke].slice(-20)); // Limit to max 20 keys
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, []);

    // Timer to fade out old keystrokes (lingering over 3 seconds)
    useEffect(() => {
        const interval = setInterval(() => {
            setKeys((prev) => {
                const now = Date.now();
                // Only keep keys pressed in the last 3000ms
                const filtered = prev.filter((k) => now - k.timestamp < 3000);
                // If nothing changed, return prev reference to avoid re-rendering
                if (filtered.length === prev.length) return prev;
                return filtered;
            });
        }, 100); // Check 10 times a second for smooth expiration

        return () => clearInterval(interval);
    }, []);

    const exitApp = () => {
        invoke("exit_app");
    };

    return (
        <div className="keystroke-bar" data-tauri-drag-region>
            <div className="keystroke-keys-container" data-tauri-drag-region>
                {keys.length > 0 ? (
                    keys.map((k, index) => {
                        // Calculate opacity based on age and position
                        const age = Date.now() - k.timestamp;
                        // Fade out as it approaches 3000ms
                        const ageOpacity = Math.max(0, 1 - age / 3000);
                        // Fade out as it reaches the beginning of the 20-key limit (index 0-9)
                        const posOpacity = Math.max(0, Math.min(1, index / 10));

                        return (
                            <span
                                key={k.id}
                                className="keystroke-key"
                                style={{ opacity: Math.min(ageOpacity, posOpacity) }}
                            >
                                {k.key}
                            </span>
                        );
                    })
                ) : (
                    <span className="keystroke-idle">⌨</span>
                )}
            </div>
            <button className="global-close-btn" onClick={exitApp} title="Exit Application">
                ×
            </button>
        </div>
    );
}
