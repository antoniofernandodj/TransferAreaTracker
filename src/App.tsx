import { useState, useEffect } from "react";
import "./style.css"

import { copyToClipboard, getSetHistoryCallback, setupEventListeners } from "./utils";


localStorage.clear()


function App() {
  const [transferArea, setTransferArea] = useState<string[]>([]);
  const [activeHistory, setActiveHistory] = useState<number>(0);
  const setHistory = getSetHistoryCallback(setActiveHistory);

  useEffect(() => { setupEventListeners(setTransferArea, setActiveHistory) }, [])

  function SelectHistoryButton (i: number) {

    return (

      <button
        key={i}
        type="button"
        onClick={() => setHistory(i)}
      >
        hist{i}
      </button>

    )

  }

  return (
    <div className="container">
      <p>Histórico ativo: {activeHistory}</p>

      <form className="row">
        <div>
          {Array.from({ length: 10 }).map((_, i) => (
            SelectHistoryButton(i)
          ))}
        </div>
      </form>

      <div className="transfer-area">
        <h2>Área de Transferência:</h2>
        <div className="cards-container">
          {transferArea.length > 0 ? (
            transferArea.map((item, index) => (
              <div className="card" key={index}>
                <p>{item}</p>
                <button className="copy-button" onClick={() => copyToClipboard(item)}>
                  Copiar
                </button>
              </div>
            ))
          ) : (
            <div className="card">
              <p>Não há dados disponíveis</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
