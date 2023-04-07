<script>
  let messages = [];
  let currentMessage = '';
  let apiKey = 'Replace this with your API key "sk-..."';
  let openaiApiKey = ''; // Replace with your OpenAI API key

  async function sendMessage() {
  if (!currentMessage.trim()) return;

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
}

  function updateApiKey() {
    openaiApiKey = apiKey;
    apiKey = '';
  }
  
</script>
<div class="api-key-input">
  <label for="api-key-field">API Key:</label>
  <input type="text" id="api-key-field" bind:value={apiKey} />
  <button on:click={updateApiKey}>Update</button>
</div>

  <div class="chat-container">
    <div class="chat-messages">
      {#each messages as message}
        <div class="chat-message {message.role}">
          <div class="chat-message-content">{message.content}</div>
        </div>
      {/each}
    </div>
    <div class="chat-input">
      <form on:submit|preventDefault={sendMessage}>
        <input type="text" bind:value={currentMessage} />
        <button type="submit">Send</button>
      </form>
    </div>
  </div>




<style>
  .chat-container {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .chat-messages {
    flex: 1;
    overflow-y: scroll;
  }

  .chat-message {
    display: flex;
    justify-content: flex-start;
    margin: 10px;
  }

 

  .chat-message-content {
    padding: 10px;
    border-radius: 10px;
    max-width: 80%;
    word-break: break-word;
  }

  .user .chat-message-content {
    background-color: #2ecc71;
    color: #fff;
  }

  .ai .chat-message-content {
    background-color: #f1f0f0;
    color: #333;
  }

  .chat-input {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 10px;
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
