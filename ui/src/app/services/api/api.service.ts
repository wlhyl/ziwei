import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ZiWeiRenReust } from 'src/app/interface/request-data';
import { ZiWei } from 'src/app/interface/response';

import { environment } from 'src/environments/environment';

@Injectable({
  providedIn: 'root',
})
export class ApiService {
  private readonly url = `${environment.base_url}/api`;
  private readonly http_options = {
    headers: new HttpHeaders({ 'Content-Type': 'application/json' }),
  };

  constructor(private http: HttpClient) {}

  /**
   *
   * @returns 获取紫微盘
   */
  public getZiwei(data: ZiWeiRenReust): Observable<ZiWei> {
    return this.http.post<ZiWei>(
      `${this.url}/ziwei`,
      data,
      this.http_options
    );
  }
}
