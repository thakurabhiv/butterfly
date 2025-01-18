export interface FileDetails {
    file?: File,
    name?: string,
    objectUrl?: string,
    isObjectUrlCreated: boolean,
}

export interface FooterMessage {
    message: string,
    expire?: number // milliseconds to message expiry
}

export interface User {
    username: string
    fname: string,
    mname: string,
    lname: string,
}