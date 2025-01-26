import './App.css';
import * as wasm from '../../rust-workers/pkg';

function App() {
  return (
    <div>
      <button onClick={() => wasm.greet()}>Rust func!</button>

      <button onClick={() => console.log(wasm.return_arg("Röven!"))}>Click here to see argument passing!</button>

      <button onClick={() => console.log(wasm.return_obj({name: "carro"}))}>Hömta dubbelnamn</button>
    </div>
  )
};

export default App;
