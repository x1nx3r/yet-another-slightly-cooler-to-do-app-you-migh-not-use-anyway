import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Task {
    id: string;
    text: string;
    completed: boolean;
    active: boolean;
    started_at: number | null;
}

export default function TaskWidget() {
    const [tasks, setTasks] = useState<Task[]>([]);
    const [newTask, setNewTask] = useState("");

    useEffect(() => {
        invoke<Task[]>("get_tasks").then(setTasks);
    }, []);



    const addTask = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!newTask.trim()) return;
        const updated = await invoke<Task[]>("add_task", { text: newTask });
        setTasks(updated);
        setNewTask("");

        // Auto-activate the newly added task (it will be the last one in the list)
        if (updated.length > 0) {
            const newTaskId = updated[updated.length - 1].id;
            const updatedWithActive = await invoke<Task[]>("set_active_task", { id: newTaskId });
            setTasks(updatedWithActive);
        }

    };

    const toggleTask = async (id: string) => {
        const updated = await invoke<Task[]>("toggle_task", { id });
        setTasks(updated);

    };

    const deleteTask = async (id: string) => {
        const updated = await invoke<Task[]>("delete_task", { id });
        setTasks(updated);

    };

    const activateTask = async (id: string) => {
        const updated = await invoke<Task[]>("set_active_task", { id });
        setTasks(updated);

    };

    const closeSettings = async () => {
        await invoke("toggle_settings_window");
    };

    return (
        <div className="widget-container tasks" data-tauri-drag-region>
            <div className="settings-header" data-tauri-drag-region>
                <h1>Tasks</h1>
                <button className="close-btn" onClick={closeSettings} title="Close">
                    ✕
                </button>
            </div>
            <form onSubmit={addTask}>
                <input
                    type="text"
                    value={newTask}
                    onChange={(e) => setNewTask(e.target.value)}
                    placeholder="Add task..."
                />
            </form>
            <ul>
                {tasks.map((task) => (
                    <li
                        key={task.id}
                        className={`${task.completed ? "completed" : ""} ${task.active ? "active" : ""}`}
                    >
                        <input
                            type="checkbox"
                            checked={task.completed}
                            onChange={() => toggleTask(task.id)}
                        />
                        <span onClick={() => toggleTask(task.id)}>{task.text}</span>
                        {!task.completed && (
                            <button
                                className="activate-btn"
                                onClick={() => activateTask(task.id)}
                                title={task.active ? "Stop working" : "Work on this"}
                            >
                                {task.active ? "⏸" : "▶"}
                            </button>
                        )}
                        <button onClick={() => deleteTask(task.id)}>×</button>
                    </li>
                ))}
                {tasks.length === 0 && <p className="placeholder">No tasks yet.</p>}
            </ul>
        </div>
    );
}
