const LOGIN_STATE = $state({
    isLoggedIn: false
});

type AppStateType = {
    mode?: "light" | "dark" | "system",
    dateFormat?: string,
    toastRichColors?: boolean,
    goServicePort?: number,
};
const APP_STATE: AppStateType = $state({
    mode: "light",
    dateFormat: "DD-MM-YYYY",
    toastRichColors: true,
    goServicePort: 8089,
});

export {
    LOGIN_STATE,
    APP_STATE
}