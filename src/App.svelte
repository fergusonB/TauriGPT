<script>
  import SvelteMarkdown from 'svelte-markdown'
  let messages = [];
  let currentMessage = '';
  let apiKey = 'Replace this with your API key "sk-..."';
  let openaiApiKey = '';
  let isLoading = false;
  let model = 'gpt-3.5-turbo';

  async function sendMessage() {
  if (!currentMessage.trim()) return;
  isLoading = true;

  const endpointUrl = 'https://api.openai.com/v1/chat/completions';
  const selectedModel = model;
  const messagesToSend = [{ role: 'user', content: currentMessage }];
  
  messages = [...messages, ...messagesToSend];
  currentMessage = '';

  try {
    while (true) {
    const response = await fetch(endpointUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${openaiApiKey}`
      },
      body: JSON.stringify({ model: selectedModel, messages: messagesToSend })
    });

    const data = await response.json();
    const aiMessage = { role: 'ai', content:  data.choices[0].message.content };
    messagesToSend.push(aiMessage);

    if (data.choices[0].finish_reason === 'stop') {
      break;
    }
   }
  }
  catch{
    const errorMessage = { role: 'error', content: "You don't have GPT-4 access." };
    messagesToSend.push(errorMessage);
  }
  

  messages = [...messages, ...messagesToSend.slice(1)];
  isLoading = false;

}

  function updateApiKey() {
    openaiApiKey = apiKey;
    apiKey = '';
  }

  let chatMessagesRef;

  function changeModel(event) {
    model = event.target.value;
  }

   
</script>
<div class="app">
<div class="api-key-input">
  <label for="api-key-field">API Key:</label>
  <input type="text" id="api-key-field" bind:value={apiKey} />
  <button on:click={updateApiKey}>Update</button>
  <div class="model-selector">
    <label for="model-selector">Model:</label>
    <select id="model-selector" on:change={changeModel}>
      <option value="gpt-3.5-turbo">GPT-3.5 Turbo</option>
      <option value="gpt4">GPT-4</option>
    </select>
  </div>
  
</div>

  <div class="chat-container">
    <div class="chat-messages">
      {#each messages as message}
        <div class="chat-message {message.role}">
          <div class="chat-message-content"><SvelteMarkdown source={message.content}/></div>
        </div>
      {/each}
      {#if isLoading}
      <div class="text-input__loading">
        <div class="text-input__loading--line"></div>
        <div class="text-input__loading--line"></div>
        <div class="text-input__loading--line"></div>
      </div>
      {/if}
    </div>


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
  display: flex;
  flex-direction: column;
  padding: 16px;
}

.api-key-input label,
.model-selector label {
  margin-bottom: 8px;
}

.api-key-input input,
.model-selector select {
  margin-bottom: 16px;
}

.model-selector {
  display: flex;
  flex-direction: column;
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
  padding: 0px 10px;
  margin-left: auto;
}

.ai .chat-message-content {
  background-color: #f1f0f0;
  color: #333;
  border-radius: 10px 10px 10px 0;
  padding: 0px 10px;
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

.text-input__loading--line {
  height: 10px;
  margin: 10px;
  animation: pulse 1s infinite ease-in-out;
}

/* Responsive styles */
@media (max-width: 768px) {
  .api-key-input {
    flex-direction: column;
  }

  .api-key-input input,
  .model-selector select {
    max-width: 100%;
  }
}


@keyframes pulse {
  0% {
    background-color: rgba(165, 165, 165, 0.1);
  }
  50% {
    background-color: rgba(165, 165, 165, 0.3);
  }
  100% {
    background-color: rgba(165, 165, 165, 0.1);
  }
}

    
  </style>