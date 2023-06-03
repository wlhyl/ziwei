const baseUrl = "http://127.0.0.1:8080";
const url = `${baseUrl}/api`;

export const api: { url: string; header: object; method: "POST" } = {
  url: `${url}/ziwei`,
  header: { "Content-Type": "application/json" },
  method: "POST",
};
