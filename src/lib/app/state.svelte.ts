const LOGIN_STATE = $state({
    isLoggedIn: false
});

type AppStateType = {
    theme?: "light" | "dark" | "system",
    dateFormat?: string,
    toastRichColors?: boolean,
    goServicePort?: number,
};
const APP_STATE: AppStateType = $state({
    theme: "dark",
    dateFormat: "DD-MM-YYYY hh:mm A",
    toastRichColors: true,
    goServicePort: 8080,
});

export {
    LOGIN_STATE,
    APP_STATE
}