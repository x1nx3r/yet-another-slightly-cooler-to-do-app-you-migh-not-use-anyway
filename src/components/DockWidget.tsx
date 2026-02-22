import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Task {
    id: string;
    text: string;
    completed: boolean;
    active: boolean;
    started_at: number | null;
}

export default function DockWidget() {
    const [activeTask, setActiveTask] = useState<Task | null>(null);
    const [elapsed, setElapsed] = useState("00:00");
    const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);

    const formatElapsed = (startedAt: number | null): string => {
        if (!startedAt) return "00:00";
        const now = Math.floor(Date.now() / 1000);
        const diff = Math.max(0, now - startedAt);
        const mins = Math.floor(diff / 60)
            .toString()
            .padStart(2, "0");
        const secs = (diff % 60).toString().padStart(2, "0");
        return `${mins}:${secs}`;
    };

    useEffect(() => {
        const tick = () => {
            invoke<Task | null>("get_active_task").then((task) => {
                setActiveTask(task);
                setElapsed(formatElapsed(task?.started_at ?? null));
            });
        };

        tick();
        timerRef.current = setInterval(tick, 1000);

        return () => {
            if (timerRef.current) clearInterval(timerRef.current);
        };
    }, []);

    const openSettings = async () => {
        await invoke("toggle_settings_window");
    };

    return (
        <div className="dock-bar" data-tauri-drag-region>
            <span className="dock-text" data-tauri-drag-region>
                {activeTask ? activeTask.text : "No active task"}
            </span>
            <span className="dock-sep" data-tauri-drag-region>·</span>
            <span className="dock-time" data-tauri-drag-region>
                {elapsed}
            </span>
            <button className="dock-gear" onClick={openSettings}>
                ⚙
            </button>
        </div>
    );
}
