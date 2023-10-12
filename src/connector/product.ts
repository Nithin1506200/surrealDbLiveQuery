export interface product {
  id: string;
  category: string;
  currency: string;
  description: string;
  discount: null | undefined | number;
  image_url: string;
  name: string;
  price: number;
  quantity: number;
}
