import React from 'react';
import { PartsList } from './PartsList';
import './App.css';

function App() {
  return (
    <div className="App">
      <div className='container'>
        <div className='header'>
          <h2>Add Via MPN</h2>
        </div>
        <div className='search-container'>
          <input className='search-input' type="text"  placeholder='input search query'/>
          <button className='search-btn'>Add Mock data</button>
        </div>
        <PartsList/>
      </div>
    </div>
  );
}

export default App;
