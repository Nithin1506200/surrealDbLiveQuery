import { useState } from "react";
import { product } from "./product";
import { LiveQueryResponse } from "./db.connector";

export default function useProduct() {
  const [product, setProduct] = useState<product[]>([]);

  return { product, setProduct };
}

function updateProductList(
  setProduct: React.Dispatch<React.SetStateAction<product[]>>
) {
  return (data: LiveQueryResponse<Record<string, unknown>>) => {
    const product = data.result as unknown as product;
    switch (data.action) {
      case "CREATE": {
        setProduct((prev) => [...prev, product as unknown as product]);
        break;
      }
      case "UPDATE": {
        setProduct((prev) =>
          [...prev].map((p) => {
            if (p.id === product.id) {
              return product;
            } else {
              return p;
            }
          })
        );
        break;
      }
      case "DELETE": {
        setProduct((prev) =>
          [...prev].filter((e) => e.id !== (data.result as unknown as string))
        );
        break;
      }
      case "CLOSE": {
        console.log(data);
        break;
      }
    }
  };
}
useProduct.updateProductList = updateProductList;
