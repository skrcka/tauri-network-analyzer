enum Status {
    IDLE = 'IDLE',
    LOADING = 'LOADING',
    DONE = 'DONE',
    ERROR = 'ERROR',
}

interface Metric {
    name: string;
    status: Status;
    value: number;
    fetchUrl: string;
}

interface Metrics {
    [key: string]: Metric;
}

interface State {
    status: Status;
    path: string;
}

export type {State, Metric, Metrics};
export {Status};