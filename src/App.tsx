import React from "react";
import logo from "./logo.svg";
import tauriCircles from "./tauri.svg";
import tauriWord from "./wordmark.svg";
import "./App.css";
import { invoke } from "@tauri-apps/api";
export const uploadFileInput = (
  options: Record<string, string> = {},
  filters: Function[] = []
) => {
  console.log("uploadFileInput", options);
  const input = document.createElement("input");
  input.type = "file";
  for (const k in options) {
    input.setAttribute(k, options[k]);
  }
  return new Promise<File[]>((r) => {
    input.addEventListener("change", async () => {
      const files: File[] = Array.from(input.files || []);
      r(files);
    });
    input.click();
    console.log(222);
  });
};

export const fileToBuffer = (file: Blob) => {
  return new Promise<Uint8Array | null>((r) => {
    const reader = new FileReader();
    reader.onload = () => {
      const content: any = reader.result;
      if (!content) {
        r(null);
        return;
      }
      r(new Uint8Array(content || []));
    };
    reader.readAsArrayBuffer(file);
  });
};

function App() {
  const click = async () => {
    console.log("click");
    
    let st = +new Date();
    const d = await invoke("get_data");
    console.log("get_data", +new Date() - st, d);
    const r = await invoke("to_button_internal_js", { n: 1 });
    console.log("r", r);
    const [file] = await uploadFileInput();
    const rom = (await fileToBuffer(file))!;
    console.log(rom);
    await invoke("create_nes");
    await invoke("set_rom", { rom: Array.from(rom) });
    await invoke("bootup");
    const fps = 60;
    const inv = 1000 / fps;
    const raf = (f: () => void) => {
      setTimeout(f, inv);
    };
    let f = false;
    const stepFrame = async () => {
      raf(stepFrame);
      if (f) return;
      f = true;
      let st = +new Date();
      await invoke("step_frame");
      let ed = +new Date();
      console.log("step_frame", ed - st);
      st = +new Date();
      const data = (await invoke("update_pixels")) as Uint8ClampedArray;
      ed = +new Date();
      console.log("update_pixels", ed - st, data.length);
      // console.log("data", data);
      const c = document.getElementById("nes") as HTMLCanvasElement;
      const ctx = c.getContext("2d")!;
      const w = 256;
      const h = 240;
      c.width = w;
      c.height = h;
      c.style.width = 2 * w + "px";
      c.style.height = 2 * h + "px";
      const imageData = new ImageData(new Uint8ClampedArray(data), w, h);
      ctx.putImageData(imageData, 0, 0);
      f = false;
    };
    stepFrame();

  };
  return (
    <div className="App" onClick={click}>
      <canvas id="nes"></canvas>
    </div>
  );
}

export default App;
