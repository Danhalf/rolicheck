import { createSignal } from 'solid-js';
// import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

function App() {
  const [carName, setCarName] = createSignal('');
  const [carNumber, setCarNumber] = createSignal('');
  const [fuelMain, setFuelMain] = createSignal('');
  const [fuelOther, setFuelOther] = createSignal('');

  const handleSubmit = async (e) => {
    e.preventDefault();

    // Відправка даних на бекенд для збереження
    const response = await fetch('/api/add_car', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        carName: carName(),
        carNumber: carNumber(),
        fuelMain: parseFloat(fuelMain()),
        fuelOther: parseFloat(fuelOther()),
      }),
    });

    if (response.ok) {
      // Очищення полів форми після успішного відправлення
      setCarName('');
      setCarNumber('');
      setFuelMain('');
      setFuelOther('');
    } else {
      // Обробка помилки, якщо відправка не вдалася
      console.error('Помилка при відправленні даних на бекенд');
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <div>
        <label>Назва автомобіля:</label>
        <input
          type="text"
          value={carName()}
          onInput={(e) => setCarName(e.target.value)}
        />
      </div>
      <div>
        <label>Номер автомобіля:</label>
        <input
          type="text"
          value={carNumber()}
          onInput={(e) => setCarNumber(e.target.value)}
        />
      </div>
      <div>
        <label>Витрати пального (основні):</label>
        <input
          type="number"
          value={fuelMain()}
          onInput={(e) => setFuelMain(e.target.value)}
        />
      </div>
      <div>
        <label>Витрати пального (інші):</label>
        <input
          type="number"
          value={fuelOther()}
          onInput={(e) => setFuelOther(e.target.value)}
        />
      </div>
      <button type="submit">Додати</button>
    </form>
  );
}

export default App;
