import { writable } from 'svelte/store';
import type { FooterMessage } from '$lib/types';

import { addToast } from '$lib/app/Toaster.svelte';
import type { ToastData } from '$lib/app/Toast.svelte';

enum ToastMessageType {
    ERROR = "bg-red-500",
    SUCCESS = "bg-green-500",
    WARNING = "bg-orange-500"
}

interface ToastMessage {
    type: ToastMessageType,
    title: string,
    description: string,
    color?: string
}

// store for messages to display in app footer
const FOOTER_UPDATES = writable({} as FooterMessage);
const TOAST_UPDATES = writable({} as ToastMessage);

// subscribe to toast update
TOAST_UPDATES.subscribe((message: ToastMessage) => {
    if (!message.title || !message.description) {
        return;
    }

    const toastData: ToastData = {
        title: message.title,
        description: message.description,
        color: `${message.color || message.type}`
    }

    addToast({ data: toastData });
});

export {
    FOOTER_UPDATES,
    TOAST_UPDATES,

    ToastMessageType, type ToastMessage
}
