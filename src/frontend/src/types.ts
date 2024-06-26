export interface Transaction {
  date: string;
  description: string;
  postings: Posting[];
}

interface Posting {
  account: string;
  amount: number;
}

export type SqlValue = string | number | null;

export interface TableData {
  columnNames: string[];
  data: SqlValue[][];
}

export type ChartData = Record<string, SqlValue[]>;

export interface BarChart {
  type: "barChart";
  title: string;
  xAxis: string;
  query: string;
  gridColumn: string;
  gridRow: string;
}

export interface LineChart {
  type: "lineChart";
  title: string;
  xAxis: string;
  query: string;
  gridColumn: string;
  gridRow: string;
}

export interface Text {
  type: "text";
  template: string;
  variables: Record<string, string>;
  gridColumn: string;
  gridRow: string;
}

export interface Table {
  type: "table";
  title: string;
  query: string;
  gridColumn: string;
  gridRow: string;
}

export type Component = BarChart | LineChart | Text | Table;

export interface Dashboard {
  name: string;
  components: Component[];
}

export interface Query {
  name: string;
  query: string;
}

export interface ChatResponse {
  response: string;
}
