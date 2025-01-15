import { Command, type Child } from '@tauri-apps/api/shell';
import { log } from '$lib/utils/common';

let SPAWNED_PROCESSES_MAP: { [key: string]: Child } = {}
enum SideCarStatusCode {
    OK, ALREADY_STARTED
}
type SideCarStatus = {
    code: SideCarStatusCode,
    message?: string,
    pid: number,
}

const terminateAllSideCars = () => {
    let allSideCars = Object.keys(SPAWNED_PROCESSES_MAP); 
    let sideCarLen = allSideCars.length;
    if (sideCarLen == 0) return;

    for (let sideCar of allSideCars) {
        let process = SPAWNED_PROCESSES_MAP[sideCar];
        if (process) {
            try {
                process.kill();
            } catch (ex) {
                log(`Error while terminating PID ${process.pid}: ${ex}`);
            }   
        }
    }

    SPAWNED_PROCESSES_MAP = {};
}

const startSidecar = async (sideCarName: string): Promise<SideCarStatus> => {
    const sideCar = SPAWNED_PROCESSES_MAP[sideCarName];
    if (sideCar) {
        return Promise.resolve({ code: SideCarStatusCode.ALREADY_STARTED, pid: sideCar.pid });
    }

    const command = Command.sidecar(sideCarName);
    addProcessListeners(command, sideCarName)

    try {
        let child = await command.spawn()
        SPAWNED_PROCESSES_MAP[sideCarName] = child;

        return Promise.resolve({ code: SideCarStatusCode.OK, pid: child.pid });
    } catch (ex) {
        return Promise.reject(ex);
    }
}

const addProcessListeners = (command: Command, sideCarName: string) => {
    command.on("close", () => {
        log(`[${sideCarName}] Service terminated\n`);
    })
    command.on("error", (error) => {
        log(`Error while starting ${sideCarName}: ${error}\n`)
    })

    command.stdout.on("data", (line) => {
        log(`[${sideCarName}] ${line}`)
    })
    command.stderr.on("data", (line) => {
        log(`[${sideCarName}] ${line}`)
    })
}

const terminateSidecar = async (sideCarName: string): Promise<void> => {
    const sideCarChild: Child = SPAWNED_PROCESSES_MAP[sideCarName];
    if (sideCarChild) {
        delete SPAWNED_PROCESSES_MAP[sideCarName];
    }
    
    return sideCarChild.kill();
}

export {
    startSidecar,
    terminateSidecar,
    terminateAllSideCars,
    type Child,
    type SideCarStatus,
    SideCarStatusCode
}