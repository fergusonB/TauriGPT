<script>
    import { invoke } from '@tauri-apps/api/tauri';
  
    let apiKey = '';
  
    async function handleClick() {
      await invoke('set_api_key', { apiKey });
    }
  </script>
  
  <div>
    <label for="input">Enter a value:</label>
    <input type="text" id="input" bind:value={apiKey}>
    <button on:click={handleClick}>Set Value</button>
  </div>
  