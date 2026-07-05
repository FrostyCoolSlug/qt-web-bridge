// Some basic examples of call handling

function log(message) {
    const li = document.createElement("li");
    li.textContent = message;
    document.getElementById("log").appendChild(li);
}

listen("upper-triggered", (payload) => {
    log("[JS] Event From Rust: " + JSON.stringify(payload));
})

once("upper-triggered", (payload) => {
    log("[JS] Event From Rust (once):" + JSON.stringify(payload));
})

document.getElementById("send-btn").addEventListener("click", async () => {
    const payload = document.getElementById("payload").value;
    log(`-> invokeFromJs("${payload}")`);

    emit("rust-event", {"String": "Hello from JS!"})

    await invoke("upper", {
        "text": payload,
    }).then(r => log(`<- invokeFromJs result: ${r}`))
        .catch(e => log(`<- invokeFromJs error: ${e}`));

});
