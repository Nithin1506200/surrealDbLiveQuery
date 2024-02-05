"use client";
import { resolve } from "path";
import { useEffect, useMemo, useState } from "react";

const useIndexDb = (
  name: string,
  table: string[],
  version: number = 1,
  onerror?: CallableFunction
) => {
  //   let db: undefined | IDBDatabase;
  const [db, setDb] = useState<undefined | IDBDatabase>(undefined);

  const openRequest = useMemo(
    () => indexedDB.open(name, version),
    [name, version]
  );
  useEffect(() => {
    openRequest.onupgradeneeded = function () {
      // triggers if the client had no database
      // ...perform initialization...
      alert("oni");
      let db_ = openRequest.result;
      for (let i of table) {
        if (db_.objectStoreNames.contains(i)) {
          db_.deleteObjectStore(i);
        }
        db_.createObjectStore(i, { keyPath: "id" });
      }
    };

    openRequest.onerror = function () {
      console.error("Error in db", openRequest.error);
      onerror && onerror();
    };

    openRequest.onsuccess = function () {
      setDb(openRequest.result);
      // continue working with database using db object
    };
  }, [openRequest, onerror, table]);

  function put(id: string, obj: Object, table: string) {
    let promise = new Promise((resolve: any, reject: any) => {
      let transaction = db?.transaction(table, "readwrite"); // (1)

      // get an object store to operate on it
      if (transaction) {
        let images = transaction.objectStore(table); // (2)

        let image = {
          id,
          ...obj,
        };

        let request = images.put(image); // (3)

        request.onsuccess = function () {
          // (4)

          console.log("Book added to the store", request.result);
          resolve();
        };

        request.onerror = function () {
          console.log("Error", request.error);
          reject();
        };
      }
      // resolve("fsdf");
    });
    return promise;
  }
  function get(id: string, table: string) {
    return new Promise((resolve: any, reject: any) => {
      const tx = db?.transaction(table, "readonly");
      const store = tx?.objectStore(table);
      if (store) {
        let req = store.get(id);
        req.onsuccess = (e) => {
          let result = req.result;
          if (result) {
            resolve(result);
          } else {
            reject("err");
          }
        };
        req.onerror = (e) => {
          reject(e);
        };
      } else {
        reject("rejecting as objstore doesn't exist");
      }
    });
  }
  function clear(table: string) {
    const tx = db?.transaction(table, "readwrite");
    const store = tx?.objectStore(table);
    store?.clear();
  }

  return {
    put,
    get,
    clear,
  };
};

export { useIndexDb };
