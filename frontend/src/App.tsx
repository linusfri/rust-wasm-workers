import "./App.css";
import * as wasm from "../../rust-workers/pkg";
import { v4 as uuidv4 } from "uuid";

type Size = {
  width: number;
  height: number;
};

type Panel = {
  uid: string;
  panelAreaUid: string;
  row: number;
  column: number;
  position: {
    x: number;
    y: number;
  };
  size: Size;
  panelAreaSize: Size;
  removed: boolean;
  fill: string;
};

function App() {
  function generateAlotOfPanels(): Panel[] {
    const panels: Panel[] = [];
    for (let i = 0; i < 500_000; i++) {
      panels.push({
        uid: uuidv4(),
        panelAreaUid: uuidv4(),
        row: Math.floor(Math.random() * 10),
        column: Math.floor(Math.random() * 10),
        position: {
          x: Math.floor(Math.random() * 10),
          y: Math.floor(Math.random() * 10),
        },
        size: {
          width: Math.floor(Math.random() * 10),
          height: Math.floor(Math.random() * 10),
        },
        panelAreaSize: {
          width: Math.floor(Math.random() * 100),
          height: Math.floor(Math.random() * 100),
        },
        removed: false,
        fill: "red",
      });
    }
    return panels;
  }

  const panels = generateAlotOfPanels();

  function getPanelsFourCorners(
    panels: Panel[]
  ): {
    topLeft: number[];
    topRight: number[];
    bottomRight: number[];
    bottomLeft: number[];
  }[] {
    return panels.map((panel) => {
      return {
        topLeft: [0,0],
        topRight: [0, panel.size.width],
        bottomRight: [panel.size.width, panel.size.height],
        bottomLeft: [0, panel.size.height]
      }  
    })
  }

  function timeJs() {
    const timesToRun = 10;
    const results: number[] = [];
    for (let i = 0; i < timesToRun; i++) {
      const start = performance.now();
      const panelCorners = getPanelsFourCorners(panels);
      const end = performance.now();
      results.push(end - start);
    }

    const meanResults = results.reduce((acc, current) => {
      return acc + current
    }, 0) / results.length
    
    console.log(`Mean results js: ${meanResults} ms`);
  }

  function timeRust() {
      const results: number[] = [];
      const start = performance.now();
      const panelCorners = wasm.time_rust();
      const end = performance.now();
      results.push(end - start);

    const meanResults = results.reduce((acc, current) => {
      return acc + current
    }, 0) / results.length
    console.log(`Rust time: ${meanResults} ms`);
  }

  return (
    <div>
      <button onClick={() => wasm.greet()}>Rust func!</button>

      <button onClick={() => console.log(wasm.return_arg("Röven!"))}>
        Click here to see argument passing!
      </button>

      <button onClick={() => console.log(wasm.return_obj({ name: "carro" }))}>
        Hömta dubbelnamn
      </button>

      <button
        onClick={() => {
          timeJs();
        }}
      >
        JS time
      </button>

      <button
        onClick={() => {
          timeRust();
        }}
      >
        Rust time
      </button>

      {/* {panels.map((panel) => {
        return (
          <div
            key={panel.uid}
          >
            {panel.uid}
          </div>
        );
      })} */}
    </div>
  );
}

export default App;
