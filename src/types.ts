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
  type: "chart";
  title: string;
  chartType: ChartType;
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

export type Component = Chart | Text;

export interface Dashboard {
  name: string;
  components: Component[];
}

export interface PlaidTransaction {
  account: string;
  amount: number;
  date: string;
  name: string;
}
