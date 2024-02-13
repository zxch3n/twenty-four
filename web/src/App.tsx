import { useState } from "react";
import { solve } from "wasm24";

function App() {
  const [target, setTarget] = useState(24);
  const [value, setValue] = useState("");
  const [ans, setAns] = useState("");

  return (
    <div className="dark flex min-h-[100vh] w-full flex-col items-center bg-slate-800 pt-5 text-gray-100">
      <div className="mb-4 flex flex-row">
        <img src="/icon.svg" />
        <h1 className="ml-2 text-xl">
          <input
            className="inline w-6 bg-transparent outline-none"
            onChange={(v) => {
              const value = parseInt(v.target.value);
              if (isNaN(value)) return;
              setTarget(value);
            }}
            value={target}
          />{" "}
          点计算器
        </h1>
      </div>
      <div>
        <div className="flex flex-row">
          <input
            type="text"
            className="rounded-md bg-slate-900 p-2 text-gray-100 outline-none"
            placeholder="Enter the numbers"
            value={value}
            onChange={(v) => setValue(v.target.value)}
          />
          <button
            className="ml-2 rounded bg-slate-700 p-2 hover:bg-slate-500"
            onClick={() => {
              try {
                const values = value
                  .split(" ")
                  .map((x) => x.trim())
                  .filter(Boolean)
                  .map((x) => {
                    const ans = parseInt(x);
                    if (ans >= 65536) {
                      throw "Number too large"
                    }

                    return ans
                  });
                const input = Uint16Array.from(values);
                const ans = solve(target, input);
                if (ans) {
                  setAns(ans + " = " + target);
                } else {
                  setAns("No solution found");
                }
              } catch (e) {
                setAns("Invalid input: " + e);
              }
            }}
          >
            Calculate
          </button>
        </div>

      </div>
      <div className="min-w-[280px] mt-3 text-left font-mono">{ans}</div>
    </div>
  );
}

export default App;
