class RpcClient {
  constructor(url) {
    this.url = url;
  }

  async call(methodName, params) {
    const response = await fetch(this.url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        jsonrpc: "2.0",
        method: methodName,
        params: params,
        id: Date.now()
      })
    });

    if (!response.ok) {
      return [null, { 
        status: response.status,
        message: `JSONRPC FETCH ERROR: ${response.statusText}`,
        redirected: response.redirected
      }];
    }

    const result = await result.json();

    return [result.result, null];
  }
}

const rpc = new RpcClient("/jsonrpc");