import React, { useEffect, useRef } from 'react';
import { Chart, CategoryScale, LinearScale, BarController, BarElement } from 'chart.js';

// Register the necessary chart elements and scales
Chart.register(CategoryScale, LinearScale, BarController, BarElement);

interface BarChartProps {
  data: [number, number][];
  xLabel?: string;
  yLabel?: string;
}

const BarChart: React.FC<BarChartProps> = ({ data, xLabel, yLabel }) => {
  const chartRef = useRef<HTMLCanvasElement | null>(null);
  const chartInstanceRef = useRef<Chart | null>(null);

  useEffect(() => {
    if (!chartRef.current) return;

    if (chartInstanceRef.current) {
      chartInstanceRef.current.destroy();
    }

    const ctx = chartRef.current.getContext('2d');
    if (!ctx) return;

    chartInstanceRef.current = new Chart(ctx, {
      type: 'bar',
      data: {
        labels: data.map(([label]) => label),
        datasets: [
          {
            label: 'Bar Chart',
            data: data.map(([, value]) => value),
            backgroundColor: 'blue',
          },
        ],
      },
      options: {
        responsive: true,
        scales: {
          x: {
            type: 'category',
            title: {
              display: true,
              text: xLabel ?? 'Categories',
            },
          },
          y: {
            beginAtZero: true,
            title: {
              display: true,
              text: yLabel ?? 'Values',
            },
          },
        },
      },
    });
  }, [data]);

  return <canvas ref={chartRef} />;
};

export default BarChart;
