import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import LiveConnector from "./connector/db.connector";
import useProduct from "./connector/useProduct";
import { product } from "./connector/product";

function App() {
  const [connector, setConnector] = useState<undefined | LiveConnector>();
  const [discount, setDiscount] = useState(0.5);
  const { product, setProduct } = useProduct();

  useEffect(() => {
    const success = () => {
      console.log("connection successful");
    };
    const error = () => {
      console.log("error");
    };
    const connector = new LiveConnector(
      useProduct.updateProductList(setProduct),
      { success, error }
    );
    setConnector(connector);
    return () => {
      connector.kill();
    };
  }, []);
  return (
    <>
      <div>
        sample query
        <input
          type="range"
          min={0}
          max={1}
          step={0.02}
          value={discount}
          onChange={(e) => {
            setDiscount(parseFloat(e.target.value));
          }}
        ></input>
        <div>discount : {discount}</div>
        <button
          onClick={async () => {
            if (connector?.startLiveQuery) {
              const initialResult = await connector.startLiveQuery(discount);
              setProduct(initialResult as unknown as product[]);
            }
          }}
        >
          {" "}
          start query
        </button>
        <div>
          <table>
            <tr>
              <th>id</th>
              <th>name</th>
              <th>discount</th>
              <th>price</th>
              <th>quantity</th>
            </tr>
            {product.map((p, i) => {
              return (
                <tr
                  key={p.id}
                  style={{ backgroundColor: i % 2 ? "rgba(55,55,55,.1" : "" }}
                >
                  <td>{p.id}</td>
                  <td>{p.name}</td>
                  <td>{p.discount}</td>
                  <td>{p.price}</td>
                  <td>{p.quantity}</td>
                </tr>
              );
            })}
          </table>
        </div>
      </div>
    </>
  );
}

export default App;
