const protocol = location.protocol === "https:" ? "wss://" : "ws://"
const ws = new WebSocket(`${protocol}${location.host}/ws`);
const messages = document.getElementById("messages");
const input = document.getElementById("input");
const sendButton = document.getElementById("send");

ws.onopen = () => { append("System: Connected to server", "server")};

ws.onmessage = (event) => { append(event.data, "server")};

ws.onclose = () => { append("System: Disconnected", "server")};

ws.onerror = (err) => { console.error("WebSocket error:", err)};

sendButton.onclick = sendMessage;

input.addEventListener("keypress", (e) => {if (e.key === "Enter") sendMessage();})

function sendMessage() {
    const msg = input.value.trim();
    ws.send(msg);
    append(msg, "user");
    input.value = "";
}

function append(text, cls) {
    const div = document.createElement("div");
    if (cls === "user") {div.textContent = `You: ${text}`;} else {div.textContent =  `Server: ${text}`;}
    div.className = "message ${cls}";
    messages.appendChild(div);
    messages.scrollTop = messages.scrollHeight;
}