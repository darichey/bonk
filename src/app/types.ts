export interface Transaction {
  date: string;
  description: string;
  amount: {
    cents: number;
  };
  account: string;
}
