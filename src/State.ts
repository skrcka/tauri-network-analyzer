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

interface SparseMatrix {
    [key: number]: { [key: number]: number };
}

export type {State, Metric, Metrics, SparseMatrix};
export {Status};