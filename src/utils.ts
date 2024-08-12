import { clipboard } from '@tauri-apps/api';
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';


type setHistoryType = (n: number) => Promise<void>;


export const copyToClipboard = async (text: string): Promise<void> => {
    try {
      await clipboard.writeText(text);
      alert('Texto copiado para a área de transferência!');
    } catch (error) {
      console.error('Falha ao copiar para a área de transferência:', error);
      alert('Falha ao copiar o texto.');
    }
};

  
export const getSetHistoryCallback = (setActiveHistory: Function): setHistoryType => {

    const setHistory = async (n: number) => {
      setActiveHistory(n);
      await invoke("set_history", { n: n });
    }
  
    return setHistory
  
}

function arraysEquals(arr1: any, arr2: string[]) {

  type index = string | number
  
  let c1 = arr1.length === arr2.length;
  let c2 = arr1.every((value: string, index: index) => value === arr2[Number(index)])

  return c1 && c2

}


export async function setupEventListeners(setTransferArea: Function, setActiveHistory: Function) {
  
  let started = parseInt( localStorage.getItem('started') || '0')

  if (!started) {
    invoke('send_event')
    localStorage.setItem('started', '1')
  }

  listen('active-history', (event) => {

    const area = event.payload;
    let lastArea = JSON.parse(localStorage.getItem('last-area') || '[]');

    if (!arraysEquals(area, lastArea)) {
      setTransferArea(area);
      localStorage.setItem('last-area', JSON.stringify(area));
    }

  });

  listen('active-history-number', (event) => {
    const active = Number(event.payload)
    setActiveHistory(active);
  });

}