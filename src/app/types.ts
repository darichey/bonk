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
  column_names: string[];
  data: SqlValue[][];
}

export type ChartData = Record<string, SqlValue[]>;
