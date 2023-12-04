import './App.css';
import {Container, Button, Spinner, Table} from 'reactstrap';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';

import 'bootstrap/dist/css/bootstrap.min.css'
import { useState } from 'react';
import { Metric, Metrics, State, Status } from './State';


function App() {
    const [
        state,
        setState,
    ] = useState<State>({
        status: Status.IDLE,
        path: '',
    })

    const [
        metrics,
        setMetrics,
    ] = useState<Metrics>({
        max_dg: {
            name: 'Max degree',
            status: Status.IDLE,
            value: 0,
            fetchUrl: 'get_max_dg',
        },
        avg_dg: {
            name: 'Average degree',
            status: Status.IDLE,
            value: 0,
            fetchUrl: 'get_avg_dg',
        },
        cl_ef: {
            name: 'Clustering effect',
            status: Status.IDLE,
            value: 0,
            fetchUrl: 'get_cl_ef',
        },
        avg_cm_nb: {
            name: 'Average common neighbours',
            status: Status.IDLE,
            value: 0,
            fetchUrl: 'get_avg_cm_nb',
        },
        max_cm_ng: {
            name: 'Max common neighbours',
            status: Status.IDLE,
            value: 0,
            fetchUrl: 'get_max_cm_ng'
        },
    });

    async function load_dataset(path: string) {
        setState({
            path: path,
            status: Status.LOADING,
        });
        try {
            const message = await invoke('load_dataset', {path: path});
            setState({
                ...state,
                status: Status.DONE,
            });
            console.log(message);
        } catch (e) {
            setState({
                ...state,
                status: Status.ERROR,
            });
            console.error('Error calling Rust function', e);
        }
    }

    const openFilePicker = async () => {
        try {
            const selected = await open({ directory: false, multiple: false, filters: [{name: 'tsv', extensions: ['tsv']}] });
            if (selected) {
                console.log(selected);
                load_dataset(selected instanceof Array ? selected[0] : selected);
            }
        } catch (error) {
            console.error('Error opening file dialog:', error);
        }
    };

    const fetchMetric = async (metricId: string, metric: Metric) => {
        setMetrics(prevState => ({
            ...prevState,
            [metricId]: {
                ...prevState[metricId],
                status: Status.LOADING,
            }
        }));
        try {
            const value = await invoke(metric.fetchUrl);
            const parsedValue = value as number;
            console.log(value);
            setMetrics(prevState => ({
                ...prevState,
                [metricId]: {
                    ...prevState[metricId],
                    value: parsedValue,
                    status: Status.DONE,
                }
            }));
        } catch (e) {
            console.error('Error calling Rust function', e);
            setMetrics(prevState => ({
                ...prevState,
                [metricId]: {
                    ...prevState[metricId],
                    status: Status.ERROR,
                }
            }));
        }
    }

    return (
        <div className='app p-3'>
        <Container className='mt-3 mb-3 d-flex flex-grow-1 flex-column'>
        <h1>Network analyzer tool in Rust ;)</h1>
        <div className='d-flex mt-3 flex-grow-1 justify-content-center align-items-center'>
            {state.status === Status.IDLE && (
                <Button color="primary" onClick={openFilePicker}>Load file</Button>
            )}
            {state.status === Status.LOADING && (
                <Spinner color="primary" />
            )}
            {state.status === Status.DONE && (
                <Table hover bordered>
                    <tr>
                        <th>Metric</th>
                        <th>Value</th>
                    </tr>
                    {Object.keys(metrics).map((key) => (
                        <tr key={key} style={{height: '3rem'}}>
                            <td>{metrics[key].name}</td>
                            <td>
                                {metrics[key].status === Status.DONE && metrics[key].value}
                                {metrics[key].status === Status.LOADING && <Spinner color="primary" />}
                                {metrics[key].status === Status.ERROR && 'Error'}
                                {metrics[key].status === Status.IDLE && <Button color="primary" onClick={() => fetchMetric(key, metrics[key])}>Calc</Button>}
                            </td>
                        </tr>
                    ))}
                </Table>
            )}
        </div>
        </Container>
        </div>
    );
}

export default App;
