const LOGIN_STATE = $state({
    isLoggedIn: false
});

type AppUi = {
    mode: "light" | "dark" | "system",
    dateFormat: string,
    toastRichColors: boolean,
}

type PDFService = {
    name?: string,
    port?: number
}

const APP_UI_STATE: AppUi = $state({
    mode: "dark",
    dateFormat: "DD-MM-YYYY",
    toastRichColors: true,
});

const PDF_SERVICE_STATE: PDFService = $state({});

export {
    LOGIN_STATE,
    APP_UI_STATE,
    PDF_SERVICE_STATE
}