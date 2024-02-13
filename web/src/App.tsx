import { useState } from 'react'
import { solve } from "wasm24";

function App() {
  const [target, setTarget] = useState(24)
  const [value, setValue] = useState("")
  const [ans, setAns] = useState("")

  return (
    <div className='dark bg-slate-800 w-full min-h-[100vh] flex flex-col items-center text-gray-100 pt-5'>
      <div className='mb-4 flex flex-row'>
        <img src="/icon.svg" />
        <h1 className='text-xl ml-2'>
          <input className='bg-transparent inline outline-none w-6' onChange={(v) => {
            const value = parseInt(v.target.value);
            if (isNaN(value)) return;
            setTarget(value)
          }} value={target} /> 点计算器
        </h1>
      </div>
      <div>
        <div
          className='flex flex-row'
        >
          <input
            type="text"
            className='bg-slate-900 text-gray-100 p-2 rounded-md outline-none'
            placeholder='Enter the numbers'
            value={value}
            onChange={(v) => setValue(v.target.value)}
          />
          <button className='ml-2 rounded bg-slate-700 p-2 hover:bg-slate-500' onClick={() => {
            try {

              const values = value.split(" ").map(x => x.trim()).filter(Boolean).map(x => parseInt(x));
              const input = Uint8Array.from(values);
              const ans = solve(target, input);
              if (ans) {
                setAns(ans + " = " + target);
              } else {
                setAns("No solution found");
              }
            } catch (e) {
              setAns("Invalid input " + e);
            }
          }}>Calculate</button>
        </div>

        <div className='text-left mt-3 font-mono'>
          {ans}
        </div>
      </div>
    </div>
  )
}

export default App

