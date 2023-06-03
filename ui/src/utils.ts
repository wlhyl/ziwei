export function nowDateTime(): {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
} {
  let t = new Date();
  let year = t.getFullYear();
  let month = t.getMonth() + 1;
  let day = t.getDate();
  let hour = t.getHours();
  let minute = t.getMinutes();
  let second = t.getSeconds();
  return { year, month, day, hour, minute, second };
}
