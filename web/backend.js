// This connects up the backend and sets up invoke() and event listeners.

new QWebChannel(qt.webChannelTransport, function (channel) {
    const rpcPending = new Map();

    console.log("QWebChannel ready");
    const backend = channel.objects.backend;

    if (!backend) {
        console.error("Backend not found");
        return;
    }

    // Event Emitter
    backend.eventEmitted.connect((payload) => {
        console.log("Backend event emitted:", payload);
        let msg;
        try {
            msg = JSON.parse(payload);
        } catch (e) {
            console.error("Invalid event payload:", payload);
            return;
        }

        const {event, payload: data} = msg;
        window.__dispatchBackendEvent(event, data);
    });


    // Function to emit an event to the backend
    window.emit = function (eventName, payload = {}) {
        backend.eventFromJs(JSON.stringify({
            event: eventName,
            payload
        }));
    };

    document.getElementById("status").textContent = "Connected to Rust backend.";
    document.getElementById("send-btn").disabled = false;

    // Create invoke bound to this backend, and make it globally available
    window.invoke = function invoke(command, args = {}) {
        const requestId = crypto.randomUUID();
        return new Promise((resolve, reject) => {
            rpcPending.set(requestId, {resolve, reject});

            backend.invokeFromJs(JSON.stringify({
                command,
                args,
                requestId
            }));
        });
    };
    backend.invokeResponse.connect((payload) => {
        console.log("Backend result ready:", payload);
        let msg;
        try {
            msg = JSON.parse(payload);
        } catch (e) {
            console.error("Invalid backend payload:", payload);
            return;
        }

        const {requestId, status, result, error} = msg;
        const entry = rpcPending.get(requestId);
        if (!entry) return;

        rpcPending.delete(requestId);

        if (status === "ok") {
            entry.resolve(result);
        } else {
            entry.reject(error);
        }
    });

    // Initialise anything else in the backend
    backend.init();


    console.log("Backend Systems Ready");
});

// Create event listeners, these will be bound to the backend when it's available
(() => {
    const eventListeners = new Map();

    function dispatchEvent(eventName, payload) {
        const listeners = eventListeners.get(eventName);
        if (!listeners) return;

        for (const sub of listeners) {
            try {
                sub.handler(payload);
            } catch (e) {
                console.error("event handler error:", e);
            }
        }
    }

    window.listen = function (eventName, handler) {
        if (!eventListeners.has(eventName)) {
            eventListeners.set(eventName, new Set());
        }

        const subscription = {
            id: crypto.randomUUID(),
            handler,
        };

        eventListeners.get(eventName).add(subscription);

        return () => {
            const set = eventListeners.get(eventName);
            if (!set) return;

            for (const sub of set) {
                if (sub === subscription) {
                    set.delete(sub);
                    break;
                }
            }

            if (set.size === 0) {
                eventListeners.delete(eventName);
            }
        };
    };

    // Create the listen bind for 'one' handle of the listener
    window.once = function (eventName, handler) {
        let unsubscribe = () => {
        };

        function wrapper(payload) {
            unsubscribe();
            handler(payload);
        }

        unsubscribe = window.listen(eventName, wrapper);
        return unsubscribe;
    };

    window.__dispatchBackendEvent = dispatchEvent;
})();