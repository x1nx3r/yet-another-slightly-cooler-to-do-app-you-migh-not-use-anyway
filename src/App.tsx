import { useState, useEffect } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import KeystrokeWidget from "./components/KeystrokeWidget";
import TaskWidget from "./components/TaskWidget";
import MusicWidget from "./components/MusicWidget";
import DockWidget from "./components/DockWidget";
import "./App.css";

function App() {
  const [windowLabel] = useState<string>(() => getCurrentWindow().label);

  useEffect(() => {
    // Apply system accent color
    invoke<string>("get_accent_color").then((color) => {
      document.documentElement.style.setProperty("--accent-color", color);
      if (color.startsWith("#")) {
        const r = parseInt(color.slice(1, 3), 16);
        const g = parseInt(color.slice(3, 5), 16);
        const b = parseInt(color.slice(5, 7), 16);
        document.documentElement.style.setProperty("--accent-glow", `rgba(${r}, ${g}, ${b}, 0.4)`);
        document.documentElement.style.setProperty("--accent-bg", `rgba(${r}, ${g}, ${b}, 0.2)`);
      }
    });
  }, []);

  switch (windowLabel) {
    case "main":
      return (
        <div className="unified-bar" data-tauri-drag-region>
          <div className="bar-section left" data-tauri-drag-region>
            <MusicWidget />
          </div>
          <div className="bar-section middle" data-tauri-drag-region>
            <DockWidget />
          </div>
          <div className="bar-section right" data-tauri-drag-region>
            <KeystrokeWidget />
          </div>
        </div>
      );
    case "settings":
      return <TaskWidget />;
    default:
      return <div className="container">Loading...</div>;
  }
}

export default App;
