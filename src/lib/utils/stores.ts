import { writable } from 'svelte/store';
import type { FooterMessage } from '$lib/types';
import { toast } from "svelte-sonner";

enum ToastMessageType {
    ERROR = "bg-red-500",
    SUCCESS = "bg-green-500",
    WARNING = "bg-orange-500"
}

interface ToastMessage {
    type: ToastMessageType,
    title: string,
    description: string,
    position?: 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right' | 'top-center' | 'bottom-center'
}

// store for messages to display in app footer
const FOOTER_UPDATES = writable({} as FooterMessage);
const TOAST_UPDATES = writable({} as ToastMessage);

// subscribe to toast update
TOAST_UPDATES.subscribe((message: ToastMessage) => {
    if (!message.title || !message.description) {
        return;
    }
    
    let func;
    switch (message.type) {
        case ToastMessageType.SUCCESS:
            func = toast.success;
            break;
        case ToastMessageType.WARNING:
            func = toast.warning;
            break;
        case ToastMessageType.ERROR:
            func = toast.error;
            break;
    }
    
    func(message.title, {
        description: message.description,
        position: message.position ?? 'bottom-right',
    });
});

export {
    FOOTER_UPDATES,
    TOAST_UPDATES,

    ToastMessageType, type ToastMessage
}
