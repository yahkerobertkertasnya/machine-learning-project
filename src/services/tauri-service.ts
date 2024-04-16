import { createTauRPCProxy, type Dataset } from "../../bindings";

export default class TauriService {
  private static ipc: Awaited<ReturnType<typeof createTauRPCProxy>>;

  public static async getDatasets(): Promise<Dataset[]> {
    const ipc = await this.getTauRPCProxy();
    return ipc.dataset.get_datasets();
  }

  public static async getCurrentDirectory(): Promise<string> {
    const ipc = await this.getTauRPCProxy();
    return ipc.util.get_current_dir();
  }

  private static async getTauRPCProxy() {
    if (!this.ipc) {
      this.ipc = await createTauRPCProxy();
    }
    return this.ipc;
  }
}
