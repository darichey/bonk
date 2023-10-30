import { Chart } from "../types";
import QueryBarChart from "./QueryBarChart";
import QueryLineChart from "./QueryLineChart";

export default function QueryChart({ chart }: { chart: Chart }) {
  return chart.chartType === "bar" ? (
    <QueryBarChart
      title={chart.title}
      xAxis={chart.xAxis}
      query={chart.query}
    />
  ) : chart.chartType === "line" ? (
    <QueryLineChart
      title={chart.title}
      xAxis={chart.xAxis}
      query={chart.query}
    />
  ) : (
    <div>Unknown chart type: &apos;{chart.chartType}&apos;</div>
  );
}
