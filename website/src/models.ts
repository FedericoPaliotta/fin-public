
export interface Ticker {
  symbol: string;
  fee: number;
  currentGoal: number;
  currentPercent: number;
}

export interface FinTableState {
  portfolio: Portfolio;
  columnsNames: Array<string>;
  columns: Array<string>;
}

interface Portfolio {
  stocks: Array<Ticker>;
  bonds: Array<Ticker>;
  goal_stock_percent: number;
  current_stock_percent: number;
  deviation_percent: number;
}

export interface FinPortfolioResp {
  name: string,
  tickers: Array<Ticker>;
  goal_stock_percent: number;
  actual_stock_percent: number;
  total_value: number;
  deviation_percent: number;
}


//======== buy next
export interface Action {
  id: number;
  shares: number;
  price: number;
}

export interface BuyNextResp {
  actions: Array<>;
  buy_value: number;
}
