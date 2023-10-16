import { Surreal } from "surrealdb.js";
import { LiveQueryResponse } from "../../node_modules/surrealdb.js/src/types";

class LiveConnector {
  surreal!: Surreal;
  protected connected = false;
  protected queryUUID: string | undefined = undefined;

  constructor(
    public queryUpdate: (
      data: LiveQueryResponse<Record<string, unknown>>
    ) => void,
    _callback:
      | { success: CallableFunction; error: CallableFunction }
      | undefined = undefined,
    readonly url = "http://127.0.0.1:8000/rpc",
    readonly user = "root",
    readonly pass = "root",
    readonly ns = "test",
    readonly db = "test"
  ) {
    this.surreal = new Surreal();
    this.connect();
  }
  async connect() {
    try {
      await this.surreal.connect(this.url);
      console.log(this.surreal);
      //   await this.surreal.wait();
      // Signin as a namespace, database, or root user
      const sighin = await this.surreal.signin({
        user: "root",
        pass: "root",
      });
      this.connected = true;
      console.log({ sighin });
      // Select a specific namespace / database
      await this.surreal.use({ ns: this.ns, db: this.db });
    } catch (err) {
      console.error(err);
    }
  }
  setUpdateFn(
    queryUpdate: (data: LiveQueryResponse<Record<string, unknown>>) => void
  ) {
    this.queryUpdate = queryUpdate;
  }

  async startLiveQuery(discount: number) {
    this.queryUUID && (await this.surreal.kill(this.queryUUID));
    const initalResult = await this.surreal.query(
      `SELECT * FROM product where discount >= ${discount}`
    );
    const res = await this.surreal.query(
      `LIVE SELECT * FROM product where discount >= ${discount}`
    );
    const uuid = (res[0].result as string) || "";

    await this.surreal.listenLive(uuid, (data) => {
      console.log("live update", data);
      this.queryUpdate(data);
    });

    this.queryUUID = uuid;
    console.log("inital result", uuid, initalResult[0].result);
    return initalResult[0].result;
  }
  kill() {
    // this.surreal.kill(this.queryUUID);
    this.surreal.close();
  }
}
export { type LiveQueryResponse };
export default LiveConnector;
