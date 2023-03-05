export interface Account {
  id: number;
  name: string;
}

export interface Transaction {
  id: number;
  date: string;
  description: string | null;
  source: number;
  destination: number;
  amount: number;
}
