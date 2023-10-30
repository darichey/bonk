export interface Transaction {
  date: string;
  description: string;
  amount: {
    cents: number;
  };
  account: string;
}

export type SqlValue = string | number | null;

export interface TableData {
  column_names: string[]; // todo camelCase
  data: SqlValue[][];
}

export type ChartData = Record<string, SqlValue[]>;

export type ChartType = "line" | "bar";

export interface Chart {
  title: string;
  chartType: ChartType;
  xAxis: string;
  query: string;
}

export interface Dashboard {
  name: string;
  charts: Chart[];
}
