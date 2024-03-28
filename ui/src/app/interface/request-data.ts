export interface DateRequest {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
}

/**
 * 本命星盘请求数据
 */
export interface ZiWeiRenReust {
  // 出生时间
  native_date: DateRequest;

  // 推运时间
  process_date: DateRequest;

  describe: string;
  masculine: boolean;
}
