type Merchants = "PAYPAL" | "TESLA" | "TATA";
type OrderStatus = "NEW" | "PENDING" | "UNAUTHORIZED" | "SUCCESS" | "FAILURE";
interface Orders {
  merchant: Merchants | string;
  status: OrderStatus;
  amount: number;
  created_on: string;
}
