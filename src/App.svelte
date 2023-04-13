<script>
  import SvelteMarkdown from 'svelte-markdown'
  let messages = [];
  let currentMessage = '';
  let apiKey = 'Replace this with your API key "sk-..."';
  let openaiApiKey = '';
  let isLoading = false;

  async function sendMessage() {
  if (!currentMessage.trim()) return;
  isLoading = true;

  const endpointUrl = 'https://api.openai.com/v1/chat/completions';
  const model = 'gpt-3.5-turbo';
  const messagesToSend = [{ role: 'user', content: currentMessage }];
  
  messages = [...messages, ...messagesToSend];
  currentMessage = '';

  while (true) {
    const response = await fetch(endpointUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${openaiApiKey}`
      },
      body: JSON.stringify({ model, messages: messagesToSend })
    });

    const data = await response.json();
    const aiMessage = { role: 'ai', content:  data.choices[0].message.content };
    messagesToSend.push(aiMessage);

    if (data.choices[0].finish_reason === 'stop') {
      break;
    }
  }

  messages = [...messages, ...messagesToSend.slice(1)];
  isLoading = false;

}

  function updateApiKey() {
    openaiApiKey = apiKey;
    apiKey = '';
  }

  let chatMessagesRef;


  
</script>
<div class="app">
<div class="api-key-input">
  <label for="api-key-field">API Key:</label>
  <input type="text" id="api-key-field" bind:value={apiKey} />
  <button on:click={updateApiKey}>Update</button>
</div>

  <div class="chat-container">
    <div class="chat-messages">
      {#each messages as message}
        <div class="chat-message {message.role}">
          <div class="chat-message-content"><SvelteMarkdown source={message.content}/></div>
        </div>
      {/each}
    </div>
    {#if isLoading}
    <div class="loading">Loading...</div>
    {/if}

  </div>
  <div class="chat-input">
    <form on:submit|preventDefault={sendMessage}>
      <input type="text" bind:value={currentMessage} />
      <button type="submit">Send</button>
    </form>
  </div>
</div>

  <style>
  :global(html, body) {
    height: 100%;
    margin: 0;
    font-family: Arial, sans-serif;
  }

  .app {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .api-key-input {
    padding: 16px;
  }

  .chat-container {
    flex-grow: 1;
    overflow-y: auto;
    padding: 16px;
  }

  .chat-messages {
    display: flex;
    flex-direction: column;
    padding-bottom: 72px;
  }

  .chat-message {
    display: flex;
    margin: 10px;
  }

  .user .chat-message-content {
    background-color: #2ecc71;
    color: #fff;
    border-radius: 10px 10px 0 10px;
    padding: 10px;
    margin-left: auto;
  }

  .ai .chat-message-content {
    background-color: #f1f0f0;
    color: #333;
    border-radius: 10px 10px 10px 0;
    padding: 10px;
  }

  .chat-input {
    display: flex;
    justify-content: space-between;
    align-items: center;
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    padding: 8px 16px;
  }

  .chat-input form {
    display: flex;
    flex: 1;
    margin-right: 10px;
  }

  .chat-input input {
    flex: 1;
    padding: 10px;
    border-radius: 5px;
    border: none;
    font-size: 16px;
    outline: none;
  }

  .chat-input button {
    padding: 10px;
    border-radius: 5px;
    border: none;
    background-color: #3498db;
    color: #fff;
    font-size: 16px;
    cursor: pointer;
  }
    
  </style>