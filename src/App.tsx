import { useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import KeystrokeWidget from "./components/KeystrokeWidget";
import TaskWidget from "./components/TaskWidget";
import MusicWidget from "./components/MusicWidget";
import DockWidget from "./components/DockWidget";
import "./App.css";

function App() {
  const [windowLabel] = useState<string>(() => getCurrentWindow().label);

  switch (windowLabel) {
    case "dock":
      return <DockWidget />;
    case "settings":
      return <TaskWidget />;
    case "keystrokes":
      return <KeystrokeWidget />;
    case "music":
      return <MusicWidget />;
    default:
      return <div className="container">Loading...</div>;
  }
}

export default App;
