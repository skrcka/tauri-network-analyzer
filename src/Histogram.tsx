import React, { useEffect, useRef } from 'react';
import { Chart, LinearScale, BarController, BarElement } from 'chart.js';


// Register the necessary chart elements and scales
Chart.register(LinearScale, BarController, BarElement);

interface HistogramProps {
  data: number[]; // Array of floats
  binCount?: number; // Optional number of bins
  xLabel?: string;
  yLabel?: string;
}

const Histogram: React.FC<HistogramProps> = ({ data, binCount = 10, xLabel, yLabel }) => {
  const chartRef = useRef<HTMLCanvasElement | null>(null);
  const chartInstanceRef = useRef<Chart | null>(null);

  // Function to create bins
  const createBins = (data: number[], binCount: number) => {
    const min = Math.min(...data);
    const max = Math.max(...data);
    const binSize = (max - min) / binCount;

    const bins = new Array(binCount).fill(0);
    const binLimits = Array.from({length: binCount + 1}, (_, i) => min + i * binSize);

    data.forEach(value => {
      const binIndex = Math.min(binCount - 1, Math.floor((value - min) / binSize));
      bins[binIndex]++;
    });

    return { bins, binLimits };
  };

  useEffect(() => {
    if (!chartRef.current) return;

    if (chartInstanceRef.current) {
      chartInstanceRef.current.destroy();
    }

    const ctx = chartRef.current.getContext('2d');
    if (!ctx) return;

    const { bins, binLimits } = createBins(data, binCount);

    chartInstanceRef.current = new Chart(ctx, {
      type: 'bar',
      data: {
        labels: binLimits.slice(0, -1).map((limit, i) => `${limit.toFixed(2)} - ${binLimits[i + 1].toFixed(2)}`),
        datasets: [
          {
            label: 'Histogram',
            data: bins,
            backgroundColor: 'green',
          },
        ],
      },
      options: {
        responsive: true,
        scales: {
          x: {
            title: {
              display: true,
              text: xLabel ?? 'Bins',
            },
          },
          y: {
            beginAtZero: true,
            title: {
              display: true,
              text: yLabel ?? 'Frequency',
            },
          },
        },
      },
    });
  }, [data, binCount]);

  return <canvas ref={chartRef} />;
};

export default Histogram;
