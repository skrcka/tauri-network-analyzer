import './App.css';
import {Container, Button, Spinner, Table, Row, Col, Input} from 'reactstrap';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';

import 'bootstrap/dist/css/bootstrap.min.css'
import { useState } from 'react';
import { Metric, Metrics, SparseMatrix, State, Status } from './State';
import BarChart from './Chart';
import GraphVisualizer from './GraphVisualizer';
import Histogram from './Histogram';


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
        nodes: {
            name: 'Nodes',
            status: Status.IDLE,
            value: 0,
            fetchUrl: 'get_node_count',
        },
        edges: {
            name: 'Edges',
            status: Status.IDLE,
            value: 0,
            fetchUrl: 'get_edge_count',
        },
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
        avg_cl_coef: {
            name: 'Average clustering coefficient',
            status: Status.IDLE,
            value: 0,
            fetchUrl: 'get_avg_cl_coef',
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

    const [
        dgDistributionLoading,
        setDgDistributionLoading,
    ] = useState<boolean>(false);

    const [
        dgDistribution,
        setDgDistribution,
    ] = useState<Array<[number, number]>>([]);

    const [
        clEffectDistributionLoading,
        setClEffectDistributionLoading,
    ] = useState<boolean>(false);

    const [
        clEffectDistribution,
        setClEffectDistribution,
    ] = useState<Array<[number, number]>>([]);

    const [
        clCoefficientLoading,
        setCoefficientLoading,
    ] = useState<boolean>(false);

    const [
        clCoefficientDistribution,
        setCoefficientDistribution,
    ] = useState<Array<[number, number]>>([]);

    const [
        clCoefficientDistributionBins,
        setCoefficientDistributionBins,
    ] = useState<number>(0);

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

    const fetchDgDistribution = async () => {
        try {
            setDgDistributionLoading(true);
            const value = await invoke('get_dg_dis');
            const parsedValue = value as Array<[number, number]>;
            console.log(value);
            setDgDistribution(parsedValue);
            setDgDistributionLoading(false);
        } catch (e) {
            console.error('Error calling Rust function', e);
        }
    }

    const fetchClEffectDistribution = async () => {
        try {
            setClEffectDistributionLoading(true);
            const value = await invoke('get_cl_ef_dis');
            const parsedValue = value as Array<[number, number]>;
            console.log(value);
            setClEffectDistribution(parsedValue);
            setClEffectDistributionLoading(false);
        } catch (e) {
            console.error('Error calling Rust function', e);
        }
    }

    const fetchClCoefficientDistribution = async () => {
        try {
            setCoefficientLoading(true);
            const value = await invoke('get_cl_coef_dis', {bins: clCoefficientDistributionBins});
            const parsedValue = value as Array<[number, number]>;
            console.log(value);
            setCoefficientDistribution(parsedValue);
            setCoefficientLoading(false);
        } catch (e) {
            console.error('Error calling Rust function', e);
        }
    }

    const [
        node1,
        setNode1,
    ] = useState<number>(0);

    const [
        node2,
        setNode2,
    ] = useState<number>(0);

    const [
        distance,
        setDistance,
    ] = useState<number>(0);

    const [
        distanceStatus,
        setDistanceStatus,
    ] = useState<Status>(Status.IDLE);

    const fetchDistance = async () => {
        try {
            setDistanceStatus(Status.LOADING);
            const value = await invoke('djikstra', {start: node1, end: node2});
            const parsedValue = value as number;
            console.log(value);
            setDistance(parsedValue);
            setDistanceStatus(Status.DONE);
        } catch (e) {
            console.error('Error calling Rust function', e);
            setDistanceStatus(Status.ERROR);
        }
    }

    const [
        pathStatus,
        setPathStatus,
    ] = useState<Status>(Status.IDLE);

    const [
        graph,
        setGraph,
    ] = useState<SparseMatrix>({});

    const [
        path,
        setPath,
    ] = useState<Array<number>>([]);

    const fetchPath = async () => {
        try {
            setPathStatus(Status.LOADING);
            const value = await invoke('djikstra_path', {start: node1, end: node2});
            console.log(value);
            const parsedValue = value as Array<SparseMatrix | Array<number>>;
            setGraph(parsedValue[0] as SparseMatrix);
            setPath(parsedValue[1] as Array<number>);
            setPathStatus(Status.DONE);
        } catch (e) {
            setPathStatus(Status.ERROR);
            console.error('Error calling Rust function', e);
            setDistanceStatus(Status.ERROR);
        }
    }

    const [
        bestStartNodeCount,
        setBestStartNodeCount,
    ] = useState<number>(5)

    const [
        bestStartNodeCountStatus,
        setBestStartNodeCountStatus,
    ] = useState<Status>(Status.IDLE)

    const [
        bestStartNodes,
        setBestStartNodes,
    ] = useState<number[]>([])

    const fetchInfluenceStart = async () => {
        try {
            setBestStartNodeCountStatus(Status.LOADING);
            const value = await invoke('get_best_starting_nodes', {n: bestStartNodeCount});
            console.log(value);
            const parsedValue = value as Array<number>;
            setBestStartNodes(parsedValue);
            setBestStartNodeCountStatus(Status.DONE);
        } catch (e) {
            setBestStartNodeCountStatus(Status.ERROR);
            console.error('Error calling Rust function', e);
        }
    }

    const [
        startNode,
        setStartNode,
    ] = useState<number[] | null>(null)

    const [
        influence,
        setInfluence,
    ] = useState<number[]>([])

    const [
        influenceStatus,
        setInfluenceStatus,
    ] = useState<Status>(Status.IDLE)

    const fetchInfluence = async () => {
        try {
            setInfluenceStatus(Status.LOADING);
            const value = await invoke('simulate_influnce_spread', {initial_nodes: [startNode]});
            console.log(value);
            const parsedValue = value as Array<SparseMatrix | Array<number>>;
            //setGraph(parsedValue[0] as SparseMatrix);
            setInfluence(parsedValue[1] as Array<number>);
            setInfluenceStatus(Status.DONE);
        } catch (e) {
            setInfluenceStatus(Status.ERROR);
            console.error('Error calling Rust function', e);
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
                <div className='d-flex flex-column flex-grow-1 justify-content-center align-items-center'>
                <Table hover bordered>
                    <tr>
                        <th>Metric</th>
                        <th style={{minWidth: '10rem'}}>Value</th>
                    </tr>
                    {Object.keys(metrics).map((key) => (
                        <tr key={key} style={{height: '3rem'}}>
                            <td className='text-start'>{metrics[key].name}</td>
                            <td className='text-end'>
                                {metrics[key].status === Status.DONE && metrics[key].value}
                                {metrics[key].status === Status.LOADING && <Spinner color="primary" />}
                                {metrics[key].status === Status.ERROR && 'Error'}
                                {metrics[key].status === Status.IDLE && <Button color="primary" onClick={() => fetchMetric(key, metrics[key])}>Calc</Button>}
                            </td>
                        </tr>
                    ))}
                </Table>
                <Row className='w-100 mt-3'>
                    <Col className='d-flex justify-content-center w-100'>
                        <div className='w-100'>
                        <h2>Degree distribution</h2>
                        {(dgDistribution.length === 0 && !dgDistributionLoading) &&
                            <Button color="primary" onClick={fetchDgDistribution}>Show degree distribution</Button>
                        }
                        {dgDistributionLoading &&
                            <Spinner color="primary" />
                        }
                        {dgDistribution.length > 0 &&
                            <BarChart xLabel='Degree' yLabel='Count' data={dgDistribution} />
                        }
                        </div>
                    </Col>
                    <Col className='d-flex justify-content-center w-100'>
                        <div className='w-100'>
                        <h2>Clustering effect distribution</h2>
                        {(clEffectDistribution.length === 0 && !clEffectDistributionLoading) &&
                            <Button color="primary" onClick={fetchClEffectDistribution}>Show clustering effect distribution</Button>
                        }
                        {clEffectDistributionLoading &&
                            <Spinner color="primary" />
                        }
                        {clEffectDistribution.length > 0 &&
                            <BarChart xLabel='Degree' yLabel='Clustering effect' data={clEffectDistribution} />
                        }
                        </div>
                    </Col>
                </Row>
                <Row className='w-100 mt-3'>
                    <Col className='d-flex justify-content-center w-100'>
                    <div className='w-100'>
                        <h2>Clustering coefficient distribution</h2>
                        {(clCoefficientDistribution.length === 0 && !clCoefficientLoading) &&
                            <div>
                                <Input
                                    type="number"
                                    name="bins"
                                    id="bins"
                                    placeholder="Bins"
                                    className='d-inline-block me-3'
                                    style={{width: '10rem'}}
                                    onChange={(evnt)=>{
                                        setCoefficientDistributionBins(parseInt(evnt.target.value))
                                    }}
                                />
                                <Button color="primary" onClick={fetchClCoefficientDistribution}>Show clustering effect distribution</Button>
                            </div>
                        }
                        {clCoefficientLoading &&
                            <Spinner color="primary" />
                        }
                        {clCoefficientDistribution.length > 0 &&
                            <BarChart xLabel='Clustering coefficient' yLabel='Count' data={clCoefficientDistribution} />
                        }
                        </div>
                    </Col>
                </Row>
                <Row className='w-100 mt-3' style={{height: '7rem'}}>
                    <h2>Calculate distance of nodes</h2>
                    <Col className='d-flex justify-content-center w-100'>
                        <Input
                            type="number"
                            name="node1"
                            id="node1"
                            placeholder="Node 1"
                            onChange={(evnt)=>{
                                setDistanceStatus(Status.IDLE)
                                setPathStatus(Status.IDLE)
                                setNode1(parseInt(evnt.target.value))
                            }}
                        />
                    </Col>
                    <Col className='d-flex justify-content-center w-100'>
                        <Input
                            type="number"
                            name="node2"
                            id="node2"
                            placeholder="Node 2"
                            onChange={(evnt)=>{
                                setDistanceStatus(Status.IDLE)
                                setPathStatus(Status.IDLE)
                                setNode2(parseInt(evnt.target.value))
                            }}
                        />
                    </Col>
                    <Col className='d-flex justify-content-center w-100'>
                    {distanceStatus === Status.DONE && (
                        <h3>Distance: {distance}</h3>
                    )}
                    {distanceStatus === Status.LOADING && (
                        <Spinner color="primary" />
                    )}
                    {distanceStatus === Status.ERROR && (
                        <p>Error</p>
                    )}
                    {distanceStatus === Status.IDLE && (
                        <Button color="primary" onClick={fetchDistance}>Calculate</Button>
                    )}
                    </Col>
                    <Col className='d-flex justify-content-center w-100'>
                    {pathStatus === Status.LOADING && (
                        <Spinner color="primary" />
                    )}
                    {pathStatus === Status.ERROR && (
                        <p>Error</p>
                    )}
                    {pathStatus === Status.IDLE && (
                        <Button color="primary" onClick={fetchPath}>Visualize</Button>
                    )}
                    </Col>
                </Row>
                <Row className='w-100 mt-3'>
                    <div>
                    {pathStatus === Status.DONE && (
                        <GraphVisualizer sparseMatrix={graph} path={path} />
                    )}
                    </div>
                </Row>
                <Row className='w-100 mt-3'>
                    <h2>Simulate influence spread</h2>
                    <Col className='d-flex justify-content-center w-100'>
                        <Input
                            type='number'
                            defaultValue={bestStartNodeCount}
                            onChange={(event) => {
                                setBestStartNodeCount(Number(event.target.value));
                            }}
                            placeholder='Start node count'
                        />
                    </Col>
                    <Col className='d-flex justify-content-center w-100'>
                        <Button
                            onClick={fetchInfluenceStart}
                        >
                            Get starting point for n nums
                        </Button>
                    </Col>
                    <Col className='w-100 mt-3'>
                        <div>
                        {bestStartNodeCountStatus === Status.LOADING &&
                            <Spinner color="primary" />
                        }
                        {bestStartNodeCountStatus === Status.DONE &&
                            bestStartNodes.join(',')
                        }
                        </div>
                    </Col>
                </Row>
                <Row className='w-100 mt-3'>
                    <Col className='d-flex justify-content-center w-100'>
                        <Input
                            type='number'
                            onChange={(event) => {
                                if(event.target.value === ''){
                                    setStartNode(null);
                                }
                                else{
                                    setStartNode(event.target.value.split(',').map(n => Number(n)))
                                }
                            }}
                            placeholder='Comma separated or leave blank for random'
                        />

                    </Col>
                    <Col className='w-100 mt-3'>
                        <Button
                            onClick={fetchInfluence}
                        >
                            Get influence
                        </Button>
                    </Col>
                    <Col className='w-100 mt-3'>
                        <div>
                        {influenceStatus === Status.LOADING &&
                            <Spinner color="primary" />
                        }
                        {influenceStatus === Status.DONE &&
                            influence.length
                        }
                        </div>
                    </Col>
                </Row>
                </div>
            )}
        </div>
        </Container>
        </div>
    );
}

export default App;
