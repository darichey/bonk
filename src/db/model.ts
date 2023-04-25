export interface Account {
  id: number;
  name: string;
}

export interface Transaction {
  id: number;
  date: string;
  description: string | null;
  account: number;
  amount: number;
}
