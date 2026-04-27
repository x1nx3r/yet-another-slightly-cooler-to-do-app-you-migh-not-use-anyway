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
      // Generate a glow color (accent with 40% opacity)
      // This is a simple HEX to RGBA conversion for the glow effect
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
